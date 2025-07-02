use bevy::prelude::*;

use super::OBSTACLE_SPAWN_TIME;

#[derive(Resource)]
pub struct ObstacleSpawnTimer {
    pub timer: Timer,
}

impl Default for ObstacleSpawnTimer {
    fn default() -> Self {
        ObstacleSpawnTimer {
            timer: Timer::from_seconds(OBSTACLE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
