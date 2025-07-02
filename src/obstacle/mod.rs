use bevy::prelude::*;
use resources::ObstacleSpawnTimer;
use systems::{spawn_obstacles_over_time, tick_obstacle_spawn_timer};

pub mod components;
mod resources;
mod systems;

pub const OBSTACLE_SPAWN_TIME: f32 = 2.0;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleSpawnTimer>().add_systems(
            Update,
            (tick_obstacle_spawn_timer, spawn_obstacles_over_time),
        );
    }
}
