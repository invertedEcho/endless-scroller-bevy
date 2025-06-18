use crate::resources::WindowDimensions;
use background_image::BackgroundImagePlugin;
use bevy::prelude::*;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use states::AppState;
use systems::{on_resize_system, spawn_camera};

mod background_image;
mod components;
mod physics;
mod player;
mod resources;
mod states;
mod systems;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(BackgroundImagePlugin)
        .add_plugins(PhysicsPlugin)
        .init_state::<AppState>()
        .init_resource::<WindowDimensions>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_resize_system)
        .run();
}
