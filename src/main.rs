use crate::resources::WindowDimensions;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use enemy::EnemyPlugin;
use ground::GroundPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use scrolling_background::ScrollingBackgroundPlugin;
use states::GameState;
use systems::{on_resize_system, spawn_camera};
use ui::UiPlugin;

mod components;
mod enemy;
mod ground;
mod player;
mod resources;
mod score;
mod scrolling_background;
mod states;
mod systems;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(UiPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScrollingBackgroundPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(ScorePlugin)
        .init_state::<GameState>()
        .init_resource::<WindowDimensions>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, on_resize_system)
        .run();
}
