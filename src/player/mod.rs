use bevy::prelude::*;
use events::JumpPlayerEvent;
use systems::{jump_player_system, spawn_player};

pub mod components;
pub mod events;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, jump_player_system)
            .add_event::<JumpPlayerEvent>();
    }
}
