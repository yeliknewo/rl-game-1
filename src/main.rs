#[macro_use]
extern crate log;

extern crate core;
extern crate dependencies;

pub use dependencies::{env_logger};

fn main() {
    env_logger::init().unwrap_or_else(|err| panic!("unable to initiate env logger: {}", err));

    core::start();
    warn!("game exited successfully");
}
