use bevy::prelude::*;
use resources::ScoreResource;
use systems::{check_if_player_beyond_enemy, reset_score_resource};

use crate::states::GameState;

pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScoreResource>()
            .add_systems(
                Update,
                check_if_player_beyond_enemy.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Playing), reset_score_resource);
    }
}
