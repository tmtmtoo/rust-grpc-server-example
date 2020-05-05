use super::*;
use crate::component::*;
use crate::service::query_error::*;
use anyhow::*;
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error as ThisError;

#[derive(new, Debug)]
pub struct SayHelloUseCaseRequest<'a> {
    greeting: &'a model::Greeting,
}

impl<'a> Into<SaveGreetingRequest<'a>> for &'a SayHelloUseCaseRequest<'a> {
    fn into(self) -> SaveGreetingRequest<'a> {
        SaveGreetingRequest::new(&self.greeting)
    }
}

#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum SayHelloUseCaseError {
    #[error("failed to handle storing: {0}")]
    FailedToHandleStoring(QueryError),
}

pub type SayHelloUseCaseResult = Result<model::Message, SayHelloUseCaseError>;

#[derive(new)]
pub struct SayHelloUseCase<R> {
    repository: Arc<R>,
}

#[async_trait]
impl<'a, RQ: 'a, R> Component<'a, RQ, SayHelloUseCaseResult> for SayHelloUseCase<R>
where
    RQ: Send + Sync,
    &'a RQ: Into<SayHelloUseCaseRequest<'a>>,
    for<'r> R: Component<'r, SayHelloUseCaseRequest<'r>, SaveGreetingResult>,
{
    async fn handle(&self, request: &'a RQ) -> SayHelloUseCaseResult {
        self.repository
            .handle(&request.into())
            .await
            .map(|_| request.into().greeting.hello())
            .map_err(SayHelloUseCaseError::FailedToHandleStoring)
    }
}

#[derive(new)]
pub struct SayHelloUseCaseStub {
    output: SayHelloUseCaseResult,
}

#[async_trait]
impl<'a, RQ: 'a> Component<'a, RQ, SayHelloUseCaseResult> for SayHelloUseCaseStub
where
    RQ: Send + Sync,
    &'a RQ: Into<SayHelloUseCaseRequest<'a>>,
{
    async fn handle(&self, _: &'a RQ) -> SayHelloUseCaseResult {
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handle_ok() {
        let stub = Arc::new(StubRepository::new(Ok(())));
        let usecase = SayHelloUseCase::new(stub);
        let greeting = model::Greeting::try_new("foo").unwrap();
        let message = usecase.handle(&greeting).await.unwrap();
        let content = message.as_ref();
        assert!(content.rfind("hello foo").is_some())
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
        let actual = usecase.handle(&greeting).await.unwrap_err();
        let expected = SayHelloUseCaseError::FailedToHandleStoring(query_error.clone());
        assert_eq!(actual, expected);
    }
}
