use engine::GameState;

pub trait Bot {
    fn tick(&mut self, game_state: &GameState);
}

impl<F> Bot for F
where
    F: FnMut(&GameState),
{
    fn tick(&mut self, game_state: &GameState) {
        self(game_state)
    }
}
