use std::fmt::{self, Debug, Formatter};

use connection::Connection;
use bot::Bot;
use errors::*;


/// The game engine object in charge of coordinating bots and communicating
/// with the server.
pub struct Engine {
    conn: Connection,
    game_state: GameState,
    bots: Vec<Box<Bot>>,
}

impl Engine {
    pub fn new() -> Result<Engine> {
        let conn = Connection::connect()?;
        let game_state = GameState::default();
        let bots = Vec::new();

        Ok(Engine {
            conn,
            game_state,
            bots,
        })
    }

    pub fn with_bot<B: Bot + 'static>(mut self, bot: B) -> Engine {
        self.bots.push(Box::new(bot));
        self
    }
}

impl Debug for Engine {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Engine")
            .field("conn", &self.conn)
            .field("game_state", &self.game_state)
            .finish()
    }
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct GameState;
