// use errors::*;
use sc2api::{response, Response};

use prost::Message;
use ws::{self, connect, Handler, Handshake};

use std::error::Error;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

pub struct Connection {
    recv_ch: Receiver<response::Response>,
    conn_thread: JoinHandle<()>,
}

pub struct Client {
    tx: Sender<response::Response>,
    out: ws::Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<(), ws::Error> {
        println!("on_open");

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<(), ws::Error> {
        if let Ok(r) = Response::decode(msg.into_data()) {
            self.tx.send(r.response.unwrap()).unwrap();
        };

        Ok(())
    }
}

impl Connection {
    pub fn connect() -> Result<(Connection), Box<Error>> {
        let (recv_tx, recv_rx) = channel();

        let thread = thread::Builder::new()
            .name("StarCraft connection".into())
            .spawn(move || {
                connect("ws://127.0.0.1:5000/sc2api", |out| {
                    Client {
                        tx: recv_tx.clone(),
                        out: out,
                    }
                }).unwrap()
            }).unwrap();

        let connection = Connection {
            recv_ch: recv_rx,
            conn_thread: thread,
        };

        Ok(connection)
    }

    pub fn recv_response(&mut self) -> Result<(response::Response), Box<Error>> {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        match self.recv_ch.recv() {
            Ok(r) => Ok(r),
            Err(_) => { panic!() }
        }
    }
}
