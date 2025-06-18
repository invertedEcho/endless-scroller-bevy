use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::RelevantForDespawnOnResize, resources::WindowDimensions, utils::get_y_of_ground,
};

use super::{components::Knight, events::RedrawKnightEvent};

pub fn spawn_knight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let y_of_ground = get_y_of_ground(window_dimensions.height);
    println!("y_of_ground: {}", y_of_ground);

    // Our knight
    commands
        .spawn((
            RigidBody::Dynamic,
            Sprite {
                image: asset_server.load("sprites/knight_single.png"),
                ..default()
            },
            Knight {},
            RelevantForDespawnOnResize {},
        ))
        .insert(Collider::ball(10.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, y_of_ground, 0.0));
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

pub fn jump_knight_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut knight_query: Query<&mut Transform, With<Knight>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        // let mut knight = knight_query.single_mut().expect("Only one knight exists");
    }
}
