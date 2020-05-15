use super::*;
use crate::component::*;
use crate::service::query_error::*;
use anyhow::*;
use async_trait::async_trait;
use derive_more::AsRef;
use std::sync::Arc;
use thiserror::Error as ThisError;

#[derive(new, Debug, AsRef)]
pub struct SayHelloUseCaseRequest {
    greeting: model::Greeting,
}

#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum SayHelloUseCaseError {
    #[error("failed to handle storing: {0}")]
    FailedToHandleStoring(QueryError),
}

pub type SayHelloUseCaseResult = Result<model::Message, SayHelloUseCaseError>;

#[derive(new)]
pub struct SayHelloUseCase {
    repository: Arc<dyn Component<model::Greeting, QueryResult<()>>>,
}

#[async_trait]
impl<RQ> Component<RQ, SayHelloUseCaseResult> for SayHelloUseCase
where
    RQ: AsRef<SayHelloUseCaseRequest> + Send + Sync,
{
    async fn handle(&self, request: &RQ) -> SayHelloUseCaseResult {
        self.repository
            .handle(request.as_ref().as_ref())
            .await
            .map(|_| request.as_ref().greeting.hello())
            .map_err(SayHelloUseCaseError::FailedToHandleStoring)
    }
}

#[cfg(test)]
#[derive(new)]
pub struct SayHelloUseCaseStub {
    output: SayHelloUseCaseResult,
}

#[cfg(test)]
#[async_trait]
impl<RQ> Component<RQ, SayHelloUseCaseResult> for SayHelloUseCaseStub
where
    RQ: AsRef<SayHelloUseCaseRequest> + Send + Sync,
{
    async fn handle(&self, _: &RQ) -> SayHelloUseCaseResult {
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{repository::*, *};

    impl AsRef<SayHelloUseCaseRequest> for SayHelloUseCaseRequest {
        fn as_ref(&self) -> &SayHelloUseCaseRequest {
            self
        }
    }

    #[tokio::test]
    async fn handle_ok() {
        let stub = Arc::new(StubRepository::new(Ok(())));
        let usecase = SayHelloUseCase::new(stub);
        let greeting = model::Greeting::try_new("foo").unwrap();
        let request = SayHelloUseCaseRequest::new(greeting);
        let result = usecase.handle(&request).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_err() {
        let query_error = QueryError::new(
            QueryErrorKind::FailedToConnectStore,
            std::io::Error::from(std::io::ErrorKind::TimedOut),
        );
        let stub = Arc::new(StubRepository::new(Err(query_error.clone())));
        let usecase = SayHelloUseCase::new(stub);
        let greeting = model::Greeting::try_new("foo").unwrap();
        let request = SayHelloUseCaseRequest::new(greeting);
        let actual = usecase.handle(&request).await.unwrap_err();
        let expected = SayHelloUseCaseError::FailedToHandleStoring(query_error.clone());
        assert_eq!(actual, expected);
    }
}
