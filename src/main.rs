use crate::resources::WindowDimensions;
use background_image::BackgroundImagePlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use events::RelevantForMoveYEvent;
use obstacles::ObstaclesPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use states::AppState;
use systems::{handle_relevant_for_move_y_event, on_resize_system, scrolling_system};

mod background_image;
mod camera;
mod components;
mod events;
mod obstacles;
mod physics;
pub mod player;
mod resources;
mod states;
mod systems;
mod utils;

// TODO: we need to detect when an image is off bounds and put it in front of all images again

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Adds a system that prints diagnostics to the console.
            // The other diagnostics plugins can still be used without this if you want to use them in an ingame overlay for example.
            // LogDiagnosticsPlugin::default(),
            // Adds frame time, FPS and frame count diagnostics.
            // FrameTimeDiagnosticsPlugin::default(),
            // Adds an entity count diagnostic.
            // bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // Adds cpu and memory usage diagnostics for systems and the entire game process.
            // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            CameraPlugin,
            PhysicsPlugin,
            PlayerPlugin,
            BackgroundImagePlugin,
            ObstaclesPlugin,
        ))
        .init_state::<AppState>()
        .init_resource::<WindowDimensions>()
        .add_systems(
            Update,
            (
                on_resize_system,
                scrolling_system,
                handle_relevant_for_move_y_event,
            ),
        )
        .add_event::<RelevantForMoveYEvent>()
        .run();
}
