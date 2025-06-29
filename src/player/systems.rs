use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{resources::WindowDimensions, utils::get_y_of_ground};

use super::components::Player;

const PLAYER_COLLIDER_RADIUS: f32 = 10.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let y_of_player = 0.0;
    // let y_of_ground = get_y_of_ground(window_dimensions.height);
    // println!("y_of_ground: {}", y_of_ground);

    commands.spawn((
        RigidBody::Dynamic,
        Sprite {
            image: asset_server.load("sprites/knight_single.png"),
            ..default()
        },
        Player,
        Collider::ball(PLAYER_COLLIDER_RADIUS),
        Transform::from_xyz(0.0, y_of_player, 10.0),
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
    ));
}

pub fn jump_knight_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut knight_velocity: Query<(&mut Velocity, &Transform), With<Player>>,
    window_dimensions: Res<WindowDimensions>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        println!("Space pressed!");
        let (mut velocity, transform) = knight_velocity
            .single_mut()
            .expect("Exactly one knight exists with velocity");

        let y_of_ground = get_y_of_ground(window_dimensions.height);
        let bottom_position_of_knight = transform.translation.y - PLAYER_COLLIDER_RADIUS;

        println!("y_of_ground as i32: {}", y_of_ground);
        println!(
            "bottom_position_of_knight as i32: {}",
            bottom_position_of_knight
        );

        if y_of_ground as i32 == bottom_position_of_knight as i32 {
            velocity.linvel = Vec2::new(0.0, 300.0);
        }
    }
}
