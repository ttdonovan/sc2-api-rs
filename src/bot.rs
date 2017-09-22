use engine::GameState;

/// The trait implemented by a single actor in the game.
pub trait Bot {
    /// Callback called at every game tick so the bot can inspect the state
    /// of the world and queue new actions.
    fn tick(&mut self, game_state: &GameState) -> Option<Action>;
    fn game_start(&mut self) {}
    fn game_end(&mut self) {}
}

impl<F> Bot for F
where
    F: FnMut(&GameState) -> Option<Action>,
{
    fn tick(&mut self, game_state: &GameState) -> Option<Action> {
        self(game_state)
    }
}


/// An in-game action.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Quit,
}
