mod proto {
    tonic::include_proto!("greet");
}

use crate::component::*;
use anyhow::*;
use proto::greet_server::Greet;

pub use proto::greet_server::GreetServer;
pub use proto::{HelloReply, HelloRequest};
pub use tonic::{Request, Response, Status};

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
