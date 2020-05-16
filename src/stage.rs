use crate::infrastructure::{db::*, grpc::*};
use crate::service::*;
use std::sync::Arc;

pub trait ComponentBuilder {
    fn grpc_route(&self) -> Route;
}

#[derive(new)]
pub struct Test {
    pool: ConnectionPool,
}

impl ComponentBuilder for Test {
    fn grpc_route(&self) -> Route {
        Route {
            say_hello: Box::new(greet_service::SayHelloController::new(Box::new(
                greet_service::SayHelloUseCase::new(Box::new(greet_service::Adaptor::new(
                    TransactionManager::new(self.pool.clone()),
                ))),
            ))),
        }
    }
}

#[derive(new)]
pub struct Dev {
    pool: ConnectionPool,
}

impl ComponentBuilder for Dev {
    fn grpc_route(&self) -> Route {
        let adaptor = Arc::new(greet_service::Adaptor::new(TransactionManager::new(
            self.pool.clone(),
        )));

        Route {
            say_hello: Box::new(WithPerf::new(
                "measurement say_hello",
                WithLogging::new(
                    "say_hello controller",
                    greet_service::SayHelloController::new(Box::new(WithLogging::new(
                        "say_hello usecase",
                        greet_service::SayHelloUseCase::new(Box::new(WithLoggingByShared::new(
                            "save greeting",
                            adaptor,
                        ))),
                    ))),
                ),
            )),
        }
    }
}
