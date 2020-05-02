use super::*;
use anyhow::*;
use std::marker::PhantomData;

#[derive(new)]
struct WithLogging<RQ, RS, ER, C>
where
    RQ: std::fmt::Debug + Send + Sync,
    RS: std::fmt::Debug + Send + Sync,
    ER: std::fmt::Debug + Send + Sync,
    for<'a> C: Component<'a, RQ, Result<RS, ER>>,
{
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
    _er: PhantomData<ER>,
}

#[async_trait]
impl<'b, RQ, RS, ER, C> Component<'b, RQ, Result<RS, ER>> for WithLogging<RQ, RS, ER, C>
where
    RQ: std::fmt::Debug + Send + Sync,
    RS: std::fmt::Debug + Send + Sync,
    ER: std::fmt::Debug + Send + Sync,
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
