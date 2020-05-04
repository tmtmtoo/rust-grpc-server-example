use super::model;
use super::usecase::*;
use crate::component::*;
use crate::infrastructure::grpc::{HelloReply, HelloRequest};
use anyhow::*;
use async_trait::*;
use std::convert::TryFrom;
use tonic::{Request, Response, Status};

impl TryFrom<&HelloRequest> for model::Greeting {
    type Error = Status;

    fn try_from(value: &HelloRequest) -> Result<Self, Self::Error> {
        model::Greeting::try_new(value.name.as_str())
            .map_err(|e| Status::invalid_argument(format!("{}", e)))
    }
}

impl<'a> Into<GreetUseCaseRequest<'a>> for &'a model::Greeting {
    fn into(self) -> GreetUseCaseRequest<'a> {
        GreetUseCaseRequest::new(self)
    }
}

#[derive(new)]
pub struct GreetController<U> {
    usecase: U,
}

#[async_trait]
impl<'a, U> Component<'a, Request<HelloRequest>, Result<Response<HelloReply>, Status>>
    for GreetController<U>
where
    for<'u> U: Component<'u, model::Greeting, GreetUseCaseResult>,
{
    async fn handle(
        &self,
        request: &'a Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let greeting = model::Greeting::try_from(request.get_ref())?;

        self.usecase
            .handle(&greeting)
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
}
