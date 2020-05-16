use async_trait::async_trait;

#[async_trait]
pub trait Component<RQ, RS>: Send + Sync + 'static {
    async fn handle(&self, request: &RQ) -> RS;
}
