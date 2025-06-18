use crate::resources::WindowDimensions;
use background_image::BackgroundImagePlugin;
use bevy::prelude::*;
use player::PlayerPlugin;
use states::AppState;
use systems::{on_resize_system, spawn_camera};

mod background_image;
mod components;
mod player;
mod resources;
mod states;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(BackgroundImagePlugin)
        .init_state::<AppState>()
        .init_resource::<WindowDimensions>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_resize_system)
        .run();
}
