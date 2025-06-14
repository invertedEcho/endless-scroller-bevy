use bevy::prelude::*;
use systems::{
    handle_scrolling_background, on_resize_system, redraw_background_system, spawn_camera,
    spawn_entire_background_image_from_layers, spawn_knight,
};

mod components;
mod systems;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Default, Copy)]
pub enum GameState {
    #[default]
    RUNNING,
    PAUSED,
    LOADING,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct AppState {
    game_state: GameState,
}

#[derive(Resource)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        WindowDimensions {
            width: 1280.0,
            height: 720.0,
        }
    }
}

#[derive(Event)]
pub struct RedrawBackgroundEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_event::<RedrawBackgroundEvent>()
        .init_resource::<WindowDimensions>()
        .add_systems(
            Startup,
            (
                spawn_entire_background_image_from_layers,
                spawn_knight,
                spawn_camera,
            ),
        )
        .add_systems(
            Update,
            (
                // handle_scrolling_background,
                on_resize_system,
                redraw_background_system,
            ),
        )
        .run();
}
