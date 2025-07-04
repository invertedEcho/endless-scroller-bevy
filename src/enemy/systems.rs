use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::RelevantForMoveY, enemy::components::Enemy, resources::WindowDimensions,
    states::GameState, utils::get_y_of_ground,
};

use super::{
    SLIME_GREEN_SINGLE_SPRITE_REL_PATH, resources::EnemySpawnTimer,
    utils::get_image_size_enemy_sprite,
};

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_dimensions: Res<WindowDimensions>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let y_of_ground = get_y_of_ground(window_dimensions.height);

        let image_size = get_image_size_enemy_sprite();

        let slime_green_sprite_bevy_path = SLIME_GREEN_SINGLE_SPRITE_REL_PATH
            .split("/")
            .collect::<Vec<&str>>()[1..]
            .join("/");

        let scaled_image_width = (image_size.width * 2) as f32;
        let scaled_image_height = (image_size.height * 2) as f32;

        commands.spawn((
            Sprite {
                image: asset_server.load(slime_green_sprite_bevy_path),
                custom_size: Some(vec2(scaled_image_width, scaled_image_height)),
                ..default()
            },
            Transform::from_xyz(150.0, y_of_ground + image_size.height as f32, 0.0),
            Collider::cuboid(scaled_image_width / 2.0, scaled_image_height / 2.0),
            Enemy::default(),
            ActiveEvents::COLLISION_EVENTS,
            RelevantForMoveY,
        ));
    }
}

pub fn handle_collision_event(
    mut reader: EventReader<CollisionEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in reader.read() {
        match event {
            CollisionEvent::Started(_, _, _) => {
                // TODO: Not all collision events mean player is dead
                next_game_state.set(GameState::Dead);
            }
            _ => {}
        }
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy in enemy_query {
        commands.entity(enemy).despawn();
    }
}
