// #[macro_use] extern crate error_chain;
#[macro_use]
extern crate error_chain;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate ws;

mod connection;
mod errors;
mod engine;
mod bot;

pub use connection::Connection;
pub use engine::{Engine, GameState};
pub use bot::{Action, ActionSet, Bot};


pub mod sc2api {
    include!(concat!(env!("OUT_DIR"), "/sc2api_protocol.rs"));
}
