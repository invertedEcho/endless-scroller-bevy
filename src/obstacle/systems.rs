use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    obstacle::components::Platform, resources::WindowDimensions, states::GameState,
    utils::get_y_of_ground,
};

use super::resources::ObstacleSpawnTimer;

const PLATFORM_GREEN_SINGLE_SPRITE_REL_PATH: &str = "assets/sprites/platform_green_single.png";
const SLIME_GREEN_SINGLE_SPRITE_REL_PATH: &str = "assets/sprites/slime_green_single.png";

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

        let image_size = imagesize::size(SLIME_GREEN_SINGLE_SPRITE_REL_PATH)
            .expect("Can get imagesize of single platform sprite");

        let platform_sprite_bevy_path = SLIME_GREEN_SINGLE_SPRITE_REL_PATH
            .split("/")
            .collect::<Vec<&str>>()[1..]
            .join("/");

        let scaled_image_width = (image_size.width * 2) as f32;
        let scaled_image_height = (image_size.height * 2) as f32;

        commands.spawn((
            Sprite {
                image: asset_server.load(platform_sprite_bevy_path),
                custom_size: Some(vec2(scaled_image_width, scaled_image_height)),
                ..default()
            },
            Transform::from_xyz(50.0, y_of_ground + image_size.height as f32, 0.0),
            Collider::cuboid(scaled_image_width / 2.0, scaled_image_height / 2.0),
            Platform,
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

pub fn handle_collision_event(
    mut reader: EventReader<CollisionEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in reader.read() {
        println!("collision event: {:?}", event);
        next_game_state.set(GameState::DEAD);
    }
}
