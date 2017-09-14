// use errors::*;
use sc2api::Response;

use prost::Message;
use ws::{self, connect, Handler, Handshake, Sender};

use std::error::Error;
use std::thread;

pub struct Connection {
    // conn_thread: thread::JoinHandle<()>,
}

pub struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        println!("on_open");

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        if let Ok(response) = Response::decode(msg.into_data()) {
            println!("{:?}", response);
        };

        Ok(())
    }
}

impl Connection {
    pub fn connect() -> Result<(), Box<Error>> {
        let thread = thread::Builder::new()
            .name("StarCraft connection".into())
            .spawn(move || {
                connect("ws://127.0.0.1:5000/sc2api", |out| {
                    Client {
                        out: out
                    }
                }).unwrap()
            }).unwrap();

        let _ = thread.join();

        Ok(())
    }
}
