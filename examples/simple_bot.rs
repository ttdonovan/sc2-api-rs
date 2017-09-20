extern crate sc2_api;

use sc2_api::{ActionSet, Engine, GameState};

fn main() {
    let mut engine = Engine::new()
        .expect("Couldn't create a new game engine")
        .with_bot(|game_state: &GameState| {
            println!("The current game state is: {:?}", game_state);
            ActionSet::empty()
        });

    engine.run();
}
