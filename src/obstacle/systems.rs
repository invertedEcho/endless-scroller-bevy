use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    obstacle::components::Platform, player::PLAYER_COLLIDER_RADIUS, resources::WindowDimensions,
    utils::get_y_of_ground,
};

use super::resources::ObstacleSpawnTimer;

const SINGLE_PLATFORM_SPRITE_REL_PATH: &str = "assets/sprites/platform_green_single.png";

pub fn tick_obstacle_spawn_timer(
    mut obstacle_spawn_timer: ResMut<ObstacleSpawnTimer>,
    time: Res<Time>,
) {
    obstacle_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_obstacles_over_time(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_dimensions: Res<WindowDimensions>,
    obstacle_spawn_timer: Res<ObstacleSpawnTimer>,
) {
    if obstacle_spawn_timer.timer.finished() {
        let y_of_ground = get_y_of_ground(window_dimensions.height);

        let image_size = imagesize::size(SINGLE_PLATFORM_SPRITE_REL_PATH)
            .expect("Can get imagesize of single platform sprite");

        let platform_sprite_bevy_path = SINGLE_PLATFORM_SPRITE_REL_PATH
            .split("/")
            .collect::<Vec<&str>>()[1..]
            .join("/");
        commands.spawn((
            Sprite {
                image: asset_server.load(platform_sprite_bevy_path),
                ..default()
            },
            Transform::from_xyz(50.0, y_of_ground + PLAYER_COLLIDER_RADIUS, 0.0),
            Collider::cuboid(
                (image_size.width / 2) as f32,
                (image_size.height / 2) as f32,
            ),
            Platform,
        ));
    }
}
