use anyhow::*;
use async_trait::async_trait;
use std::marker::PhantomData;

#[async_trait]
pub trait Component<RQ, RS> {
    async fn handle(&self, request: &RQ) -> RS;
}

#[derive(new)]
struct WithLogging<RQ, RS, ER, C>
where
    RQ: std::fmt::Debug + Send + Sync,
    RS: std::fmt::Debug + Send + Sync,
    ER: std::fmt::Debug + Send + Sync,
    C: Component<RQ, Result<RS, ER>> + Send + Sync,
{
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
    _er: PhantomData<ER>,
}

#[async_trait]
impl<RQ, RS, ER, C> Component<RQ, Result<RS, ER>> for WithLogging<RQ, RS, ER, C>
where
    RQ: std::fmt::Debug + Send + Sync,
    RS: std::fmt::Debug + Send + Sync,
    ER: std::fmt::Debug + Send + Sync,
    C: Component<RQ, Result<RS, ER>> + Send + Sync,
{
    async fn handle(&self, request: &RQ) -> Result<RS, ER> {
        self.inner
            .handle(request)
            .await
            .map(|response| {
                debug!(
                    "{}: request: {:?}, response: {:?}",
                    self.name, request, response
                );
                response
            })
            .map_err(|error| {
                warn!("{}: request: {:?}, error: {:?}", self.name, request, error);
                error
            })
    }
}

#[derive(new)]
struct WithPerf<RQ, RS, C>
where
    RQ: Send + Sync,
    RS: Send + Sync,
    C: Component<RQ, RS> + Send + Sync,
{
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
}

#[async_trait]
impl<RQ, RS, C> Component<RQ, RS> for WithPerf<RQ, RS, C>
where
    RQ: Send + Sync,
    RS: Send + Sync,
    C: Component<RQ, RS> + Send + Sync,
{
    async fn handle(&self, request: &RQ) -> RS {
        let now = std::time::Instant::now();

        let response = self.inner.handle(request).await;

        debug!("{}: elapsed: {:?}", self.name, now.elapsed());

        response
    }
}
