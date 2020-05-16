mod proto {
    tonic::include_proto!("greet");
}

use crate::component::*;
use anyhow::*;
use proto::greet_server::Greet;

pub use proto::greet_server::GreetServer;
pub use proto::{HelloReply, HelloRequest};
pub use tonic::{Request, Response, Status};

pub struct Route {
    pub say_hello: Box<dyn Component<Request<HelloRequest>, Result<Response<HelloReply>, Status>>>,
}

#[tonic::async_trait]
impl Greet for Route {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        self.say_hello.handle(&request).await
    }
}
