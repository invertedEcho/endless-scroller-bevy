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
    query: Query<Entity, With<Knight>>,
) {
    let y_of_ground = get_y_of_ground(window_dimensions.height);
    println!("y_of_ground: {}", y_of_ground);

    let count_of_existing_knights = query.iter().len();
    if count_of_existing_knights > 0 {
        println!(
            "WARN: spawn_knight was called even though there are already knights existing. Count: {}",
            count_of_existing_knights
        );
        println!("Skipping spawning knight...");
        return;
    }

    commands
        .spawn((
            RigidBody::Dynamic,
            Sprite {
                image: asset_server.load("sprites/knight_single.png"),
                ..default()
            },
            RelevantForDespawnOnResize {},
        ))
        .insert(Knight {})
        .insert(Collider::ball(10.0))
        .insert(Transform::from_xyz(0.0, y_of_ground, 0.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        });
}

pub fn redraw_knight_system(
    mut event_reader: EventReader<RedrawKnightEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
    query: Query<Entity, With<Knight>>,
) {
    if event_reader.read().next().is_some() {
        println!("Received redraw knight systems event, spawning new knight");
        spawn_knight(commands, asset_server, window_dimensions, query);
    }
}

pub fn jump_knight_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut knight_velocity: Query<(&mut Velocity, &Transform), With<Knight>>,
    window_dimensions: Res<WindowDimensions>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let (mut velocity, transform) = knight_velocity
            .single_mut()
            .expect("Exactly one knight exists with velocity");

        let y_of_ground = get_y_of_ground(window_dimensions.height);
        let position_of_knight = transform.translation.y;

        if y_of_ground as i32 == position_of_knight as i32 {
            velocity.linvel = Vec2::new(0.0, 300.0);
        }
    }
}
