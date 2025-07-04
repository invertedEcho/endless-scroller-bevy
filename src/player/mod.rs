use bevy::prelude::*;
use systems::{check_if_player_dead, despawn_player, jump_knight_system, spawn_player};

use crate::states::GameState;

pub mod components;
mod systems;
pub mod utils;

pub const PLAYER_SIZE_FACTOR: f32 = 1.5;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (jump_knight_system, check_if_player_dead).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), despawn_player);
    }
}
