#![cfg_attr(feature = "cargo-clippy", allow(dead_code))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[macro_use]
extern crate envconfig_derive;

mod component;
mod infrastructure;
mod schema;
mod service;
mod stage;

use anyhow::*;
use envconfig::Envconfig;
use infrastructure::*;
use thiserror::Error;
use tonic::transport::Server;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "DATABASE_URL")]
    database_url: String,
    #[envconfig(from = "SOCKET_ADDR", default = "0.0.0.0:5001")]
    socket_addr: String,
    #[envconfig(from = "TEST", default = "false")]
    is_test: bool,
}

#[derive(Error, Debug)]
enum MainError {
    #[error("invalid config: {0}")]
    InvalidConfig(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("failed to connect to db: {0}")]
    FailedToConnectDB(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("failed to migrate db: {0}")]
    FailedToMigrateDB(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("failed to run service: {0}")]
    FailedToRunService(Box<dyn std::error::Error + Send + Sync + 'static>),
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let config = Config::init().map_err(|e| MainError::InvalidConfig(Box::new(e)))?;

    let pool = db::connection_pool(config.database_url, 4)
        .map_err(|e| MainError::FailedToConnectDB(Box::new(e)))?;

    db::migration(&pool).map_err(|e| MainError::FailedToMigrateDB(Box::new(e)))?;

    let component_builder: Box<dyn stage::ComponentBuilder> = if config.is_test {
        Box::new(stage::Test::new(pool))
    } else {
        Box::new(stage::Dev::new(pool))
    };

    let addr = config
        .socket_addr
        .parse()
        .map_err(|e| MainError::InvalidConfig(Box::new(e)))?;

    info!("Service listening on {}", addr);

    Server::builder()
        .add_service(grpc::GreetServer::new(component_builder.grpc_route()))
        .serve(addr)
        .await
        .map_err(|e| MainError::FailedToRunService(Box::new(e)))?;

    Ok(())
}
