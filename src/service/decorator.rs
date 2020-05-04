use crate::component::*;
use anyhow::*;
use async_trait::*;
use std::marker::PhantomData;

#[derive(new)]
pub struct WithLogging<RQ, RS, ER, C> {
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
    _er: PhantomData<ER>,
}

#[async_trait]
impl<'b, RQ, RS, ER, C> Component<'b, RQ, Result<RS, ER>> for WithLogging<RQ, RS, ER, C>
where
    RQ: std::fmt::Debug + Send + Sync + 'static,
    RS: std::fmt::Debug + Send + Sync + 'static,
    ER: std::fmt::Debug + Send + Sync + 'static,
    for<'a> C: Component<'a, RQ, Result<RS, ER>>,
{
    async fn handle(&self, request: &'b RQ) -> Result<RS, ER> {
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
pub struct WithPerf<RQ, RS, C> {
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
}

#[async_trait]
impl<'b, RQ, RS, C> Component<'b, RQ, RS> for WithPerf<RQ, RS, C>
where
    RQ: Send + Sync + 'static,
    RS: Send + Sync + 'static,
    for<'a> C: Component<'a, RQ, RS>,
{
    async fn handle(&self, request: &'b RQ) -> RS {
        let now = std::time::Instant::now();

        let response = self.inner.handle(request).await;

        debug!("{}: elapsed: {:?}", self.name, now.elapsed());

        response
    }
}
