mod greet;

pub use greet::*;

pub mod grpc {
    tonic::include_proto!("greet");
}

use crate::component::*;
use anyhow::*;
use grpc::greet_server::Greet;
use grpc::{HelloReply, HelloRequest};
use tonic::{Request, Response, Status};

pub struct Route<G> {
    pub greet: G,
}

#[tonic::async_trait]
impl<G> Greet for Route<G>
where
    for<'a> G: Component<'a, Request<HelloRequest>, Result<Response<HelloReply>, Status>>,
{
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        self.greet.handle(&request).await
    }
}
