extern crate prost_build;

fn main() {
    let mut prost_build = prost_build::Config::new();
    prost_build.btree_map(&["."]);

    prost_build.compile_protos(&["s2client-proto/s2clientprotocol/sc2api.proto"],
                              &["s2client-proto"]).unwrap();
}
