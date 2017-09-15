extern crate sc2_api;
extern crate prost;

use sc2_api::sc2api::{self, request, Request};

use prost::Message;

fn main() {
    let mut buf = vec![];
    let request = Request { request: Some(request::Request::Ping(sc2api::RequestPing { })) };
    let _ = request.encode(&mut buf);
    println!("ping: {:?}", buf);
}
