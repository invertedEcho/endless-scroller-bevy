use bevy::prelude::*;
use events::RedrawKnightEvent;
use systems::{redraw_knight_system, spawn_knight};

mod components;
pub mod events;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_knight)
            .add_systems(Update, redraw_knight_system)
            .add_event::<RedrawKnightEvent>();
    }
}
