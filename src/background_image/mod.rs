use bevy::prelude::*;
use events::RedrawScrollingBackgroundEvent;
use systems::{redraw_background_system, spawn_first_background_tiles};

pub mod components;
pub mod events;
mod systems;

pub struct BackgroundImagePlugin;

impl Plugin for BackgroundImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_background_tiles)
            .add_systems(Update, redraw_background_system)
            .add_event::<RedrawScrollingBackgroundEvent>();
    }
}
