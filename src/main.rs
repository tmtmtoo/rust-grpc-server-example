mod component;

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;

mod domain;

fn main() {
    env_logger::init();

    println!("Hello, world!");
}
