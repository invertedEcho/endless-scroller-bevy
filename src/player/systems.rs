use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::RelevantForMoveY, resources::WindowDimensions, utils::get_y_of_ground};

use super::components::Player;

const PLAYER_BALL_RADIUS: f32 = 10.0;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    // TODO: Why can we not spawn our player at y_of_ground + PLAYER_BALL_RADIUS, doing so will
    // cause the player just fall through the collider
    let player_transform_y = 0.0;
    // let player_transform_y = get_y_of_ground(window_dimensions.height) + 10.0;
    println!("Spawning player at x: 0 y: {:?}", player_transform_y);

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/knight_single.png"),
            ..default()
        },
        RigidBody::Dynamic,
        Player {},
        Collider::ball(PLAYER_BALL_RADIUS),
        Transform::from_xyz(0.0, player_transform_y, 1.0),
        Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        },
        RelevantForMoveY,
    ));
}

pub fn jump_player_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Velocity, &Transform), With<Player>>,
    window_dimensions: Res<WindowDimensions>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let (mut velocity, transform) = player_query
            .single_mut()
            .expect("Exactly one player exists with velocity");
        println!("Current position of player: {:?}", transform);

        let y_of_ground = get_y_of_ground(window_dimensions.height);

        // Substract by radius here to get the bottom of our player
        let position_of_knight = transform.translation.y - PLAYER_BALL_RADIUS;

        println!("y_of_ground as i32: {}", y_of_ground as i32);
        println!(
            "position_of_knight (substracted PLAYER_BALL_RADIUS: {}) as i32: {}",
            PLAYER_BALL_RADIUS, position_of_knight as i32
        );

        // TODO: This shouldnt be needed theoretically. Maybe its because our ground collider y is
        // 0?
        //
        // Cast to i32 as float seems to be precise and comparison will fail
        if y_of_ground as i32 == position_of_knight as i32 {
            println!("Passed check player on ground, setting linvel");
            velocity.linvel = Vec2::new(0.0, 300.0);
        } else {
            println!("Player not on ground, ignoring jump request");
        }
    }
}
