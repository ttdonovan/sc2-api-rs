use std::fmt::{self, Debug, Display, Formatter};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use connection::Connection;
use bot::Bot;
use errors::*;


/// The game engine object in charge of coordinating bots and communicating
/// with the server.
pub struct Engine {
    conn: Connection,
    game_state: GameState,
    bots: Vec<(Token, Box<Bot>)>,
}

impl Engine {
    pub fn new() -> Result<Engine> {
        // TODO: replace the `Connection` with a trait object so we can mock it in tests
        let conn = Connection::connect()?;
        let game_state = GameState::default();
        let bots = Vec::new();

        Ok(Engine {
            conn,
            game_state,
            bots,
        })
    }

    /// Add a new bot to the bot list.
    pub fn with_bot<B: Bot + 'static>(mut self, bot: B) -> Engine {
        let tok = Token::unique();
        self.bots.push((tok, Box::new(bot)));
        self
    }

    pub fn run(&mut self) {
        self.start_game();

        while self.game_state.running {
            self.step();
        }

        self.end_game();
    }

    /// Do any necessary jobs to start the game.
    fn start_game(&mut self) {
        debug!("Creating game");

        for &mut (_, ref mut bot) in &mut self.bots {
            bot.game_start();
        }

        self.game_state.running = true;
    }

    /// Tidy up at the end of a game.
    fn end_game(&mut self) {
        debug!("Finished the game");

        for &mut (_, ref mut bot) in &mut self.bots {
            bot.game_end();
        }
    }

    fn step(&mut self) {
        self.update_game_state();

        for &mut (tok, ref mut bot) in &mut self.bots {
            let actions = bot.tick(&self.game_state);
            debug!("Bot {} executed {:?}", tok, actions);
        }
    }

    fn update_game_state(&mut self) {
        // talk to the connection and do everything necessary to update our
        // game state
        //
        // for now just stop the game so we don't have an infinite loop
        self.game_state.running = false;

        debug!("Game updated");
        debug!("{:?}", self.game_state);
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
pub struct GameState {
    running: bool,
}

/// A unique token which is associated with exactly one bot. This allows us to
/// track which bot did what.
#[derive(Debug, Copy, Clone, PartialEq)]
struct Token(usize);

impl Token {
    /// Create a new, globally unique token.
    fn unique() -> Token {
        static NEXT_TOKEN: AtomicUsize = ATOMIC_USIZE_INIT;
        let tok = NEXT_TOKEN.fetch_add(1, Ordering::Relaxed);

        Token(tok)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}