use engine::GameState;

pub trait Bot {
    fn tick(&mut self, game_state: &GameState) -> ActionSet;
    fn game_start(&mut self) {}
    fn game_end(&mut self) {}
}

impl<F> Bot for F
where
    F: FnMut(&GameState) -> ActionSet,
{
    fn tick(&mut self, game_state: &GameState) -> ActionSet {
        self(game_state)
    }
}


/// A set of queued actions to be executed on the next tick.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ActionSet {
    actions: Vec<Action>,
}

impl ActionSet {
    pub fn empty() -> ActionSet {
        ActionSet::default()
    }
}


/// A single in-game action.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Quit,
}
