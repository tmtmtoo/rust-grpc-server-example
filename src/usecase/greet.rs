use crate::component::*;
use crate::domain::*;
use anyhow::*;
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error as ThisError;

#[derive(new, Debug)]
pub struct GreetUseCaseRequest<'a> {
    greeting: &'a model::Greeting,
}

impl<'a> Into<SaveRequest<'a, model::Greeting>> for &'a GreetUseCaseRequest<'a> {
    fn into(self) -> SaveRequest<'a, model::Greeting> {
        SaveRequest::new(&self.greeting)
    }
}

#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum GreetUseCaseError {
    #[error("failed to handle storing: {0}")]
    FailedToHandleStoring(QueryError),
}

pub type GreetUseCaseResult = Result<model::Message, GreetUseCaseError>;

#[derive(new)]
pub struct GreetUseCase<R> {
    repository: Arc<R>,
}

#[async_trait]
impl<'a, RQ: 'a, R> Component<'a, RQ, GreetUseCaseResult> for GreetUseCase<R>
where
    RQ: Send + Sync,
    &'a RQ: Into<GreetUseCaseRequest<'a>>,
    for<'r> R: Component<'r, GreetUseCaseRequest<'r>, SaveResult<()>>,
{
    async fn handle(&self, request: &'a RQ) -> GreetUseCaseResult {
        self.repository
            .handle(&request.into())
            .await
            .map(|_| request.into().greeting.hello())
            .map_err(GreetUseCaseError::FailedToHandleStoring)
    }
}

#[derive(new)]
pub struct GreetUseCaseStub {
    output: GreetUseCaseResult,
}

#[async_trait]
impl<'a, RQ: 'a> Component<'a, RQ, GreetUseCaseResult> for GreetUseCaseStub
where
    RQ: Send + Sync,
    &'a RQ: Into<GreetUseCaseRequest<'a>>,
{
    async fn handle(&self, _: &'a RQ) -> GreetUseCaseResult {
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handle_ok() {
        let stub = Arc::new(StubRepository::new(Ok(())));
        let usecase = GreetUseCase::new(stub);
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
        let usecase = GreetUseCase::new(stub);
        let greeting = model::Greeting::try_new("foo").unwrap();
        let actual = usecase.handle(&greeting).await.unwrap_err();
        let expected = GreetUseCaseError::FailedToHandleStoring(query_error.clone());
        assert_eq!(actual, expected);
    }
}
