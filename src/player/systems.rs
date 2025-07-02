use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::PLAYER_COLLIDER_RADIUS, resources::WindowDimensions, states::GameState,
    utils::get_y_of_ground,
};

use super::components::Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let y_of_ground = get_y_of_ground(window_dimensions.height);
    let y_of_player = y_of_ground + PLAYER_COLLIDER_RADIUS;

    commands.spawn((
        RigidBody::Dynamic,
        Sprite {
            image: asset_server.load("sprites/knight_single.png"),
            ..default()
        },
        Player,
        Collider::ball(PLAYER_COLLIDER_RADIUS),
        LockedAxes::ROTATION_LOCKED,
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
        let (mut velocity, transform) = knight_velocity
            .single_mut()
            .expect("Exactly one knight exists with velocity");

        let y_of_ground = get_y_of_ground(window_dimensions.height);
        let bottom_position_of_knight = transform.translation.y - PLAYER_COLLIDER_RADIUS;

        let difference = (y_of_ground - bottom_position_of_knight).abs();

        // TODO: Investigate why on certain window dimensions theres a diff of one
        if difference <= 1.0 {
            velocity.linvel = Vec2::new(0.0, 300.0);
        }
    }
}

pub fn check_if_player_dead(
    player_query: Query<&Transform, With<Player>>,
    window_dimensions: Res<WindowDimensions>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let player = player_query.single().expect("Exactly one player exists");
    let player_translation_x = player.translation.x;

    let left_edge_window = -(window_dimensions.width / 2.0);

    if player_translation_x < left_edge_window {
        next_game_state.set(GameState::DEAD);
    }
}
