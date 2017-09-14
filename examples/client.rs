extern crate sc2_api;

// use std::process::Command;

fn main() {
    // let mut command = Command::new("/Applications/StarCraft II/Versions/Base56787/SC2.app/Contents/MacOS/SC2");
    // if let Ok(child) = command.arg("-listen 127.0.0.1 -port 5000 -displayMode 0").spawn() {
    //     println!("StarCraft's id is {}", child.id());
    // } else {
    //     println!("StarCraft failed to start");
    // }

    println!("make connection...");
    sc2_api::Connection::connect();
}
