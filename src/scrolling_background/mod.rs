use bevy::prelude::*;
use systems::{scroll_background_images, spawn_background_images};

use crate::states::GameState;

pub mod components;
mod systems;
mod utils;

pub struct ScrollingBackgroundPlugin;

impl Plugin for ScrollingBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background_images)
            .add_systems(Update, scroll_background_images);
    }
}
