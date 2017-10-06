extern crate sc2_api;
extern crate prost;
extern crate tungstenite;

use std::net::TcpListener;
use std::thread::spawn;

use sc2_api::raw_protobuf_api as raw_pb;
use sc2_api::raw_protobuf_api::request::Request;
use sc2_api::raw_protobuf_api::response::Response;

use prost::Message;
use tungstenite::accept;
use tungstenite::Message as wsMessage;

fn create_game_response() -> raw_pb::Response {
    raw_pb::Response {
        error: vec![],
        status: Some(0),
        response: Some(Response::CreateGame(raw_pb::ResponseCreateGame {
            ..Default::default()
        })),
    }
}

fn join_game_response() -> raw_pb::Response {
    raw_pb::Response {
        error: vec![],
        status: Some(0),
        response: Some(Response::JoinGame(raw_pb::ResponseJoinGame {
            player_id: Some(1),
            ..Default::default()
        })),
    }
}

fn ping_response() -> raw_pb::Response {
    raw_pb::Response {
        error: vec![],
        status: Some(0),
        response: Some(Response::Ping(raw_pb::ResponsePing {
            ..Default::default()
        })),
    }
}

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
                            Request::CreateGame(_) => {
                                println!("create game...");
                                let mut buf = vec![];
                                let response = create_game_response();
                                response.encode(&mut buf).expect("failed to encode...");
                                websocket.write_message(wsMessage::binary(buf)).unwrap();
                            },
                            Request::JoinGame(_) => {
                                println!("join game...");
                                let mut buf = vec![];
                                let response = join_game_response();
                                response.encode(&mut buf).expect("failed to encode...");
                                websocket.write_message(wsMessage::binary(buf)).unwrap();
                            },
                            _ => {
                                println!("{:?}", request);
                                let mut buf = vec![];
                                let response = ping_response();
                                response.encode(&mut buf).expect("failed to encode...");
                                websocket.write_message(wsMessage::binary(buf)).unwrap();
                            },
                        }
                    }
                }
            }
        });
    }
}
