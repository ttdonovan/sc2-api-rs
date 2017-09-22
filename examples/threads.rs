use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time;

#[derive(Debug)]
struct Connection {
    recv_ch: Receiver<String>,
    send_ch: Sender<String>,
    recv_thread: JoinHandle<()>,
    send_thread: JoinHandle<()>,
}

impl Connection {
    fn connect() -> Self {
        // FIXME: replace with `rust-websocket` ClientBuilder `split()`
        let (client_tx, client_rx) = channel();

        // ignore this thread - faking messages from a remote "server"
        let server_tx = client_tx.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                server_tx.send("...server chatter...".into()).unwrap();
                thread::sleep(time::Duration::from_secs(1));
            };
        });

        // channel for connection (recv) from websocket
        let (recv_tx, recv_rx) = channel();
        // a receving connection thread that is always listening for incoming server messages
        let recv_thread = thread::spawn(move || {
            loop {
                // FIXME: replaced with `rust-websocket` `client_tx.incoming_messages()`
                let msg = match client_rx.recv() {
                    Ok(m) => m,
                    _ => panic!(),
                };

                recv_tx.send(msg).unwrap();
            }
        });

        // channel for connection (send) to websocket
        let (send_tx, send_rx) = channel();
        // a sending connection thread thats is always listening for outgoing server messages
        let send_thread = thread::spawn(move || {
            loop {
                let msg = match send_rx.recv() {
                    Ok(m) => m,
                    _ => panic!(),
                };

                println!("CLIENT SAYS: {}", msg);
                // FIXME: how to send to back to server?
                // maybe it doesn't matter if using `rust-websocket`
                // add `client_rx.send_message()`
            }
        });

        Connection {
            recv_ch: recv_rx,
            send_ch: send_tx,
            recv_thread: recv_thread,
            send_thread: send_thread,
        }
    }

    fn resv_response(&self) -> String {
        match self.recv_ch.recv() {
            Ok(m) => m,
            _ => panic!(),
        }
    }

    fn send_request(&self, msg: String) {
        self.send_ch.send(msg).unwrap();
    }
}

fn main() {
    let conn = Connection::connect();
    println!("{:?}", conn);

    // main loop
    loop {
        let msg = conn.resv_response();
        println!("SERVER SAID: {}", msg);
        conn.send_request("...roger...".into());
    }
}
