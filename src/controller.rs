pub mod grpc {
    tonic::include_proto!("greet");
}

use crate::component::*;
use crate::domain::model;
use crate::usecase::*;
use anyhow::*;
use grpc::greet_server::Greet;
use grpc::{HelloReply, HelloRequest};
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
pub struct GreetController<U>
where
    for<'a> U: Component<'a, model::Greeting, GreetUseCaseResult>,
{
    usecase: U,
}

#[tonic::async_trait]
impl<U> Greet for GreetController<U>
where
    for<'a> U: Component<'a, model::Greeting, GreetUseCaseResult>,
{
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
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
