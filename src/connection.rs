use errors::*;
use raw_protobuf_api::{response, Response};

use prost::Message;
use websocket as ws;
use websocket::client::ClientBuilder;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

const CONNECTION: &'static str = "ws://127.0.0.1:5000/sc2api";

#[derive(Debug)]
pub struct Connection {
    // recv_ch: Receiver<response::Response>,
    send_ch: Sender<(ws::OwnedMessage)>,
    pub recv_thread: JoinHandle<()>,
    pub send_thread: JoinHandle<()>,
}

impl Connection {
    pub fn connect() -> Result<Connection> {
        let client = ClientBuilder::new(CONNECTION)
                        .unwrap()
                        .connect_insecure()
                        .unwrap();

        let (mut recv_tx, mut recv_rx) = client.split().unwrap();
        let (send_tx, send_rx) = channel();

        let tx = send_tx.clone();
        let recv_thread = thread::spawn(move || {
            for message in recv_tx.incoming_messages() {
                let msg = match message {
                    Ok(m) => m,
                    Err(e) => {
                        debug!("Receive Loop: {:?}", e);
                        let _ = tx.send(ws::OwnedMessage::Close(None));
                        return;
                    },
                };

                match msg {
                    ws::OwnedMessage::Close(_) => {
                        let _ = tx.send(ws::OwnedMessage::Close(None));
                        return;
                    },
                    _  => {
                        // TODO: need send message to `tx` for `send_rx`?
                        println!("Receive Loop...");
                        debug!("{:?}", msg);
                    },
                }
            }
        });

        // TODO: maybe this thread is not needed...
        let send_thread = thread::spawn(move || {
            loop {
                let msg = match send_rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        debug!("Send Loop: {:?}", e);
                        return;
                    },
                };

                match recv_rx.send_message(&msg) {
                    Ok(()) => (),
                    Err(e) => {
                        debug!("Send Loop: {:?}", e);
                        let _ = recv_rx.send_message(&ws::Message::close());
                        return;
                    },
                }
            }
        });

        let connection = Connection {
            // recv_ch: recv_rx,
            send_ch: send_tx,
            recv_thread: recv_thread,
            send_thread: send_thread,
        };

        Ok(connection)
    }

    pub fn recv_response(&mut self) {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        // match self.recv_ch.recv() {
        //     Ok(r) => Ok(r),
        //     Err(_) => panic!(),
        // }
    }

    // FIXME: dummy function that only sends a `OwnedMessage::Close(None)`
    pub fn send_request(&self) {
        self.send_ch.send(ws::OwnedMessage::Close(None)).expect("failed to send...");
    }
}
