use super::*;
use std::marker::PhantomData;

#[derive(new)]
struct WithPerf<RQ, RS, C>
where
    RQ: Send + Sync,
    RS: Send + Sync,
    for<'a> C: Component<'a, RQ, RS>,
{
    name: &'static str,
    inner: C,
    _rq: PhantomData<RQ>,
    _rs: PhantomData<RS>,
}

#[async_trait]
impl<'b, RQ, RS, C> Component<'b, RQ, RS> for WithPerf<RQ, RS, C>
where
    RQ: Send + Sync,
    RS: Send + Sync,
    for<'a> C: Component<'a, RQ, RS>,
{
    async fn handle(&self, request: &'b RQ) -> RS {
        let now = std::time::Instant::now();

        let response = self.inner.handle(request).await;

        debug!("{}: elapsed: {:?}", self.name, now.elapsed());

        response
    }
}
