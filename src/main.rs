#![cfg_attr(feature = "cargo-clippy", allow(dead_code))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;

mod component;
mod infrastructure;
mod schema;
mod service;

use anyhow::*;
use infrastructure::*;
use service::*;
use std::sync::Arc;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let pool = db::connection_pool("postgres://dev@localhost:5432/dev", 4)?;

    db::migration(&pool)?;

    let adaptor = Arc::new(greet_service::Adaptor::new(db::TransactionManager::new(
        pool,
    )));

    let addr = "0.0.0.0:5001".parse()?;

    info!("Service listening on {}", addr);

    Server::builder()
        .add_service(grpc::GreetServer::new(grpc::Route {
            greet: WithLogging::new(
                "Greet Controller",
                greet_service::GreetController::new(WithLogging::new(
                    "Greet UseCase",
                    greet_service::GreetUseCase::new(adaptor.clone()),
                )),
            ),
        }))
        .serve(addr)
        .await?;

    Ok(())
}
