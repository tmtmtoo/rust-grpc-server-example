use async_trait::async_trait;

#[async_trait]
pub trait Component<'a, RQ, RS>: Send + Sync + 'static {
    async fn handle(&self, request: &'a RQ) -> RS;
}
