extern crate sc2_api;

use std::thread;
use std::time;

use sc2_api::raw_protobuf_api as raw_pb;

// $ /Applications/StarCraft\ II/Versions/Base56787/SC2.app/Contents/MacOS/SC2 -listen 127.0.0.1 -port 5000 -displayMode 0

fn main() {
    let mut conn = sc2_api::Connection::connect().expect("connect failed");

    // create game
    let create = raw_pb::Request {
        request: Some(raw_pb::request::Request::CreateGame(raw_pb::RequestCreateGame {
            player_setup: vec![raw_pb::PlayerSetup { type_: Some(1), ..Default::default() }],
            map: Some(raw_pb::request_create_game::Map::LocalMap(raw_pb::LocalMap{ map_path: Some("/Applications/StarCraft II/Maps/Melee/Simple64.SC2Map".into()), ..Default::default() })),
            ..Default::default()
        })),
    };

    // join game
    let join = raw_pb::Request {
        request: Some(raw_pb::request::Request::JoinGame(raw_pb::RequestJoinGame {
            participation: Some(raw_pb::request_join_game::Participation::Race(1)),
            options: Some(raw_pb::InterfaceOptions { raw: Some(true), score: Some(true), ..Default::default() }),
            ..Default::default()
        })),
    };

    conn.send_request(create);
    conn.send_request(join);

    for _ in 0..5 {
        match conn.recv_response() {
            Ok(r) => { println!("{:?}", r) },
            _ => { panic!("something has gone wrong") }
        };

        // ping
        let ping = raw_pb::Request {
            request: Some(raw_pb::request::Request::Ping(raw_pb::RequestPing {})),
        };
        conn.send_request(ping);

        thread::sleep(time::Duration::from_secs(2));
    }

    // quit
    let quit = raw_pb::Request {
        request: Some(raw_pb::request::Request::Quit(raw_pb::RequestQuit {})),
    };
    conn.send_request(quit);

    match conn.recv_response() {
        Ok(r) => { println!("Quit {:?}", r) },
        _ => { panic!("something has gone wrong") }
    };
}
