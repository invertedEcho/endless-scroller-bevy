use bevy::prelude::*;
use systems::{handle_scrolling_background, spawn_scrolling_backgrounds};

pub mod components;
mod systems;

pub struct ScrollingBackgroundPlugin;

impl Plugin for ScrollingBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_scrolling_backgrounds)
            .add_systems(Update, handle_scrolling_background);
    }
}
