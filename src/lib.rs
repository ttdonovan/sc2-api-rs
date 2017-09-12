extern crate prost;
#[macro_use] extern crate prost_derive;

pub mod sc2api {
    include!(concat!(env!("OUT_DIR"), "/sc2api_protocol.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
