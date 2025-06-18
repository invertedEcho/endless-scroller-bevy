use bevy::prelude::*;
use events::RedrawBackgroundEvent;
use systems::{handle_scrolling_background, redraw_background_system, spawn_background_image};

mod components;
pub mod events;
mod systems;

pub struct BackgroundImagePlugin;

impl Plugin for BackgroundImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background_image)
            .add_systems(
                Update,
                (redraw_background_system, handle_scrolling_background),
            )
            .add_event::<RedrawBackgroundEvent>();
    }
}
