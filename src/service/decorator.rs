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
impl<RQ, RS, ER, C> Component<RQ, Result<RS, ER>> for WithLogging<RQ, RS, ER, C>
where
    RQ: std::fmt::Debug + Send + Sync + 'static,
    RS: std::fmt::Debug + Send + Sync + 'static,
    ER: std::fmt::Debug + Send + Sync + 'static,
    C: Component<RQ, Result<RS, ER>>,
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
pub struct WithLoggingByShared<RQ, RS, ER> {
    name: &'static str,
    inner: std::sync::Arc<dyn Component<RQ, Result<RS, ER>>>,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
    _er: PhantomData<ER>,
}

#[async_trait]
impl<RQ, RS, ER> Component<RQ, Result<RS, ER>> for WithLoggingByShared<RQ, RS, ER>
where
    RQ: std::fmt::Debug + Send + Sync + 'static,
    RS: std::fmt::Debug + Send + Sync + 'static,
    ER: std::fmt::Debug + Send + Sync + 'static,
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
pub struct WithPerf<RQ, RS, C> {
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
}

#[async_trait]
impl<RQ, RS, C> Component<RQ, RS> for WithPerf<RQ, RS, C>
where
    RQ: Send + Sync + 'static,
    RS: Send + Sync + 'static,
    C: Component<RQ, RS>,
{
    async fn handle(&self, request: &RQ) -> RS {
        let now = std::time::Instant::now();

        let response = self.inner.handle(request).await;

        debug!("{}: {:?} sec", self.name, now.elapsed().as_secs_f64());

        response
    }
}
