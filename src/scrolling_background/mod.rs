use bevy::prelude::*;
use events::RedrawScrollingBackgroundEvent;
use systems::{handle_scrolling_background, spawn_scrolling_backgrounds};

pub mod components;
pub mod events;
mod systems;

pub struct ScrollingBackgroundPlugin;

impl Plugin for ScrollingBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scrolling_backgrounds)
            // .add_systems(Update, handle_scrolling_background)
            .add_event::<RedrawScrollingBackgroundEvent>();
    }
}
