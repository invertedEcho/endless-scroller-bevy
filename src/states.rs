use bevy::prelude::*;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Default, Copy)]
pub enum GameState {
    #[default]
    RUNNING,
    PAUSED,
    LOADING,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct AppState {
    game_state: GameState,
}
