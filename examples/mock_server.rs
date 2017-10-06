extern crate sc2_api;
extern crate prost;
extern crate tungstenite;

use std::net::TcpListener;
use std::thread::spawn;

use sc2_api::raw_protobuf_api as raw_pb;
use sc2_api::raw_protobuf_api::request::Request;

use prost::Message;
use tungstenite::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:5000").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap(), None).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() {
                    let req = raw_pb::Request::decode(msg.into_data()).expect("failed to decode...");

                    if let Some(request) = req.request {
                        match request {
                            Request::CreateGame(_) => println!("create game..."),
                            Request::JoinGame(_) => println!("join game..."),
                            _ => println!("{:?}", request),
                        }
                    }
                }
            }
        });
    }
}
