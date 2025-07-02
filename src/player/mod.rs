use bevy::prelude::*;
use systems::{check_if_player_dead, jump_knight_system, spawn_player};

use crate::states::GameState;

pub mod components;
mod systems;

pub const PLAYER_COLLIDER_RADIUS: f32 = 10.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, jump_knight_system)
            .add_systems(
                Update,
                check_if_player_dead.run_if(in_state(GameState::RUNNING)),
            );
    }
}
