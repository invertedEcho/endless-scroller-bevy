use bevy::prelude::*;

use crate::{components::RelevantForDespawnOnResize, resources::WindowDimensions};

use super::{components::Knight, events::RedrawKnightEvent};

pub fn spawn_knight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let window_height = window_dimensions.height;

    // TODO: I have no idea what to call these variables
    let bottom_y = -(window_height / 2.0);

    let ten_percent_of_window_height = bottom_y / 5.0;
    let one_percent_of_window_height = ten_percent_of_window_height / 10.0;

    // This is at 90.5 percent of the top of the image, but center is at 0x0, so we dont multiply
    // by 90.5, but need to substract 50.0
    let desired_thing = one_percent_of_window_height * 40.5;

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/knight_single.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, desired_thing, 1.0),
            ..default()
        },
        Knight {},
        RelevantForDespawnOnResize {},
    ));
}

pub fn redraw_knight_system(
    mut event_reader: EventReader<RedrawKnightEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    if event_reader.read().next().is_some() {
        spawn_knight(commands, asset_server, window_dimensions);
    }
}
