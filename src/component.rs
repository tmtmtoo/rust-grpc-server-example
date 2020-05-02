mod with_logging;
mod with_perf;

pub use with_logging::*;
pub use with_perf::*;

use async_trait::async_trait;

#[async_trait]
pub trait Component<'a, RQ, RS>: Send + Sync {
    async fn handle(&self, request: &'a RQ) -> RS;
}
