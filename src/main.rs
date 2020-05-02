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
mod domain;
mod gateway;
mod infrastructure;
mod schema;
mod usecase;

fn main() {
    env_logger::init();

    println!("Hello, world!");
}
