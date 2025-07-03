use crate::resources::WindowDimensions;
use bevy::prelude::*;
use obstacle::ObstaclePlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use scrolling_background::ScrollingBackgroundPlugin;
use states::GameState;
use systems::{on_resize_system, spawn_camera};

mod components;
mod obstacle;
mod physics;
mod player;
mod resources;
mod scrolling_background;
mod states;
mod systems;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScrollingBackgroundPlugin)
        .add_plugins(ObstaclePlugin)
        .init_state::<GameState>()
        .init_resource::<WindowDimensions>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_resize_system)
        .run();
}
