use bevy::prelude::*;
use resources::ObstacleSpawnTimer;
use systems::{handle_collision_event, spawn_obstacles_over_time, tick_obstacle_spawn_timer};

use crate::states::GameState;

pub mod components;
mod resources;
mod systems;

pub const OBSTACLE_SPAWN_TIME: f32 = 2.0;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ObstacleSpawnTimer>().add_systems(
            Update,
            (
                tick_obstacle_spawn_timer,
                spawn_obstacles_over_time,
                handle_collision_event,
            )
                .run_if(in_state(GameState::PLAYING)),
        );
    }
}
