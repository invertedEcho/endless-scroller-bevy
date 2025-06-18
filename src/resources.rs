use bevy::prelude::*;

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
