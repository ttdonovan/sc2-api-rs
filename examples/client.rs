extern crate sc2_api;

use sc2_api::raw_protobuf_api as raw_pb;

// use std::process::Command;

fn main() {
    // let mut command = Command::new("/Applications/StarCraft II/Versions/Base56787/SC2.app/Contents/MacOS/SC2");
    // if let Ok(child) = command.arg("-listen 127.0.0.1 -port 5000 -displayMode 0").spawn() {
    //     println!("StarCraft's id is {}", child.id());
    // } else {
    //     println!("StarCraft failed to start");
    // }

    println!("Connecting...");
    let mut conn = sc2_api::Connection::connect().expect("connect failed");

    println!("Ready.");

    // try to send a Ping request
    let req = raw_pb::Request {
        request: Some(raw_pb::request::Request::Ping(raw_pb::RequestPing {})),
    };
    conn.send_request(req);

    loop {
        match conn.recv_response() {
            Ok(r) => { println!("{:?}", r) },
            _ => { panic!("something has gone wrong") }
        };
    }
}
