use errors::*;
use raw_protobuf_api::{response, Response};

use prost::Message;
use ws::{self, connect, Handler, Handshake, Result as WsResult};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};

#[derive(Debug)]
pub struct Connection {
    recv_ch: Receiver<response::Response>,
    send_ch: Sender<()>,
    running: Arc<AtomicBool>,
    conn_thread: JoinHandle<()>,
}

pub struct Client {
    tx: Sender<response::Response>,
    // rx: Receiver<()>,
    // out: ws::Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        println!("on_open");

        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> WsResult<()> {
        if let Ok(r) = Response::decode(msg.into_data()) {
            self.tx.send(r.response.unwrap()).unwrap();
        };

        Ok(())
    }
}

impl Connection {
    pub fn connect() -> Result<Connection> {
        let (recv_tx, recv_rx) = channel();
        let (send_tx, send_rx): (Sender<()>, Receiver<()>) = channel();
        let running = Arc::new(AtomicBool::new(true));

        let thread = thread::Builder::new()
            .name("StarCraft connection".into())
            .spawn(move || {
                connect("ws://127.0.0.1:5000/sc2api", |_out| {
// // FIXME: compilation error[E0507]
// // error[E0507]: cannot move out of captured outer variable in an `FnMut` closure
// //   --> src/connection.rs:55:32
// //    |
// // 45 |         let (send_tx, send_rx): (Sender<()>, Receiver<()>) = channel();
// //    |                       ------- captured outer variable
// // ...
// // 63 |                         .spawn(move || {
// //    |                                ^^^^^^^ cannot move out of captured outer variable in an `FnMut` closure
//                     let _t = thread::Builder::new()
//                         .name("Client sender".into())
//                         .spawn(move || {
//                             // TODO: replace "loop" with "while running <AtomicBool(true)>"
//                             loop {
//                                 use std::time::Duration;
//
//                                 println!("client sender listening...");
//                                 match send_rx.recv() {
//                                     Ok(_) => println!("got a message"),
//                                     Err(_) => panic!(),
//                                 }
//
//                                 thread::sleep(Duration::from_millis(500));
//                             }
//                         })
//                         .unwrap();

                    Client {
                        tx: recv_tx.clone(),
                        // rx: send_rx,
                        // out: out,
                    }
                }).unwrap()
            })
            .unwrap();

        let connection = Connection {
            recv_ch: recv_rx,
            send_ch: send_tx,
            running: running,
            conn_thread: thread,
        };

        Ok(connection)
    }

    pub fn recv_response(&mut self) -> Result<response::Response> {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        match self.recv_ch.recv() {
            Ok(r) => Ok(r),
            Err(_) => panic!(),
        }
    }

    // TODO: accept a request and send to the channel (send_ch)
    pub fn send_request(&self) {
        self.send_ch.send(()).expect("failed to send...");
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        // TODO: do we need to wait for listening thread with send_ch (rx) to close for a graceful shutdown?
    }
}
