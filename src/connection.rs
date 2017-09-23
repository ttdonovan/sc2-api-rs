use errors::*;
use raw_protobuf_api as raw_pb;

use prost::Message;
use websocket::OwnedMessage;
use websocket::client::ClientBuilder;
use websocket::ws::dataframe::DataFrame;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

const CONNECTION: &'static str = "ws://127.0.0.1:5000/sc2api";

#[derive(Debug)]
pub struct Connection {
    recv_ch: Receiver<(OwnedMessage)>,
    send_ch: Sender<(OwnedMessage)>,
    recv_thread: JoinHandle<()>,
    send_thread: JoinHandle<()>,
}

impl Connection {
    pub fn connect() -> Result<Connection> {
        let client = ClientBuilder::new(CONNECTION)
                        .unwrap()
                        .connect_insecure()
                        .unwrap();
        let (mut client_tx, mut client_rx) = client.split().unwrap();

        let (recv_tx, recv_rx) = channel();
        let recv_thread = thread::spawn(move || {
            for message in client_tx.incoming_messages() {
                let msg = match message {
                    Ok(m) => m,
                    Err(e) => {
                        debug!("Receive Loop: {:?}", e);
                        let _ = recv_tx.send(OwnedMessage::Close(None));
                        return;
                    },
                };

                match msg {
                    OwnedMessage::Close(_) => {
                        let _ = recv_tx.send(OwnedMessage::Close(None));
                        return;
                    },
                    OwnedMessage::Binary(_)  => {
                        recv_tx.send(msg).unwrap();
                    },
                    _ => { }, // `Text(_)`, `Ping(_)` and `Pong(_)`
                }
            }
        });

        let (send_tx, send_rx) = channel();
        let send_thread = thread::spawn(move || {
            loop {
                let msg = match send_rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        debug!("Send Loop: {:?}", e);
                        return;
                    },
                };

                // FIXME: lifetime issues?
                // match client_rx.send_message(msg) {
                //     Ok(()) => (),
                //     Err(e) => {
                //         debug!("Send Loop: {:?}", e);
                //         return;
                //     }
                // }
            }
        });

        let connection = Connection {
            recv_ch: recv_rx,
            send_ch: send_tx,
            recv_thread: recv_thread,
            send_thread: send_thread,
        };

        Ok(connection)
    }

    pub fn recv_response(&mut self) -> Result<raw_pb::Response> {
        match self.recv_ch.recv() {
            Ok(m) => {
                let r = raw_pb::Response::decode(m.take_payload()).expect("failed to decode...");
                Ok(r)
            },
            _ => panic!(),
        }
    }

    pub fn send_request(&self, request: raw_pb::Request) {
        let mut buf = vec![];
        request.encode(&mut buf).expect("failed to encode...");
        self.send_ch.send(OwnedMessage::Binary(buf)).expect("failed to send...");
    }
}
