use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{resources::WindowDimensions, states::GameState, utils::get_y_of_ground};

use super::{
    components::Player,
    utils::{get_image_size_player_sprite, get_player_ball_radius},
};

const PLAYER_SPRITES_PATH: &str = "sprites/knight_single.png";

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let player_collider_ball_radius = get_player_ball_radius() as f32;
    let y_of_ground = get_y_of_ground(window_dimensions.height);
    let y_of_player = y_of_ground + player_collider_ball_radius;

    let player_sprites_size = get_image_size_player_sprite();

    commands.spawn((
        RigidBody::Dynamic,
        Sprite {
            image: asset_server.load(PLAYER_SPRITES_PATH),
            custom_size: Some(vec2(player_sprites_size.width, player_sprites_size.height)),
            ..default()
        },
        Player,
        Collider::ball(player_collider_ball_radius),
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
        let player_collider_ball_radius = get_player_ball_radius() as f32;
        let bottom_position_of_knight = transform.translation.y - player_collider_ball_radius;

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
        println!("Player dead!");
        next_game_state.set(GameState::Dead);
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player_entity = player_query.single().expect("Exactly one player exists");
    commands.entity(player_entity).despawn();
}
