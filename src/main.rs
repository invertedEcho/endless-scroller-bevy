use bevy::prelude::*;
use systems::{handle_scrolling_background, setup};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_scrolling_background)
        .run();
}
