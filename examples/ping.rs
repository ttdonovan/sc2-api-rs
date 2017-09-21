extern crate prost;
extern crate sc2_api;

use sc2_api::raw_protobuf_api::{self, request, Request, RequestPing};

use prost::Message;

fn main() {
    let mut buf = vec![];
    let request = Request {
        request: Some(request::Request::Ping(RequestPing {})),
    };
    let _ = request.encode(&mut buf);
    println!("ping: {:?}", buf);
}
