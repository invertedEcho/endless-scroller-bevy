use bevy::prelude::*;
use events::{JumpKnightEvent, RedrawKnightEvent};
use systems::{jump_knight_system, redraw_knight_system, spawn_knight};

pub mod components;
pub mod events;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_knight)
            .add_systems(Update, (redraw_knight_system, jump_knight_system))
            .add_event::<RedrawKnightEvent>()
            .add_event::<JumpKnightEvent>();
    }
}
