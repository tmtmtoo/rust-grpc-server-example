use super::model;
use super::usecase::*;
use crate::component::*;
use crate::infrastructure::grpc::{HelloReply, HelloRequest};
use anyhow::*;
use async_trait::*;
use derive_more::AsRef;
use std::convert::TryFrom;
use tonic::{Request, Response, Status};

impl TryFrom<&HelloRequest> for model::Greeting {
    type Error = Status;

    fn try_from(value: &HelloRequest) -> Result<Self, Self::Error> {
        model::Greeting::try_new(value.name.as_str())
            .map_err(|e| Status::invalid_argument(format!("{}", e)))
    }
}

#[derive(new, AsRef, Debug)]
pub struct UsecaseRequest {
    #[as_ref]
    say_hello: SayHelloUseCaseRequest,
}

#[derive(new)]
pub struct SayHelloController {
    usecase: Box<dyn Component<UsecaseRequest, SayHelloUseCaseResult>>,
}

#[async_trait]
impl Component<Request<HelloRequest>, Result<Response<HelloReply>, Status>> for SayHelloController {
    async fn handle(
        &self,
        request: &Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let greeting = model::Greeting::try_from(request.get_ref())?;
        let request = UsecaseRequest::new(SayHelloUseCaseRequest::new(greeting));

        self.usecase
            .handle(&request)
            .await
            .map(|message| {
                Response::new(HelloReply {
                    message: message.as_ref().into(),
                })
            })
            .map_err(|e| Status::internal(format!("{}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::query_error::*;

    #[tokio::test]
    async fn handle_ok() {
        let usecase = SayHelloUseCaseStub::new(Ok(model::Message::new("hello", "foo", "😄")));
        let controller = SayHelloController::new(Box::new(usecase));
        let result = controller
            .handle(&Request::new(HelloRequest { name: "foo".into() }))
            .await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn handle_err_failed_to_handle_storing() {
        let usecase = SayHelloUseCaseStub::new(Err(SayHelloUseCaseError::FailedToHandleStoring(
            QueryError::new(
                QueryErrorKind::FailedToConnectStore,
                std::io::Error::from(std::io::ErrorKind::Other),
            ),
        )));
        let controller = SayHelloController::new(Box::new(usecase));
        let result = controller
            .handle(&Request::new(HelloRequest { name: "foo".into() }))
            .await;
        assert!(result.is_err());
    }
}
