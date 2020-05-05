mod proto {
    tonic::include_proto!("greet");
}

use crate::component::*;
use anyhow::*;
use proto::greet_server::Greet;

pub use proto::greet_server::GreetServer;
pub use proto::{HelloReply, HelloRequest};
pub use tonic::{Request, Response, Status};

pub struct Route<S> {
    pub say_hello: S,
}

#[tonic::async_trait]
impl<S> Greet for Route<S>
where
    for<'a> S: Component<'a, Request<HelloRequest>, Result<Response<HelloReply>, Status>>,
{
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        self.say_hello.handle(&request).await
    }
}
