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
mod controller;
mod domain;
mod gateway;
mod infrastructure;
mod schema;
mod usecase;

use anyhow::*;
use component::*;
use controller::grpc::greet_server::GreetServer;
use controller::{GreetController, Route};
use gateway::Adaptor;
use infrastructure::db;
use std::sync::Arc;
use tonic::transport::Server;
use usecase::GreetUseCase;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let pool = db::connection_pool("postgres://dev@localhost:5432/dev", 4)?;

    db::migration(&pool)?;

    let adaptor = Arc::new(Adaptor::new(db::TransactionManager::new(pool)));

    let addr = "0.0.0.0:5001".parse()?;

    info!("Greet Service listening on {}", addr);

    Server::builder()
        .add_service(GreetServer::new(Route {
            greet: WithLogging::new(
                "greet controller",
                GreetController::new(WithLogging::new(
                    "greet usecase",
                    GreetUseCase::new(adaptor.clone()),
                )),
            ),
        }))
        .serve(addr)
        .await?;

    Ok(())
}
