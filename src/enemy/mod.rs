use bevy::prelude::*;
use resources::EnemySpawnTimer;
use systems::{
    despawn_enemies, handle_collision_event, spawn_enemies_over_time, tick_enemy_spawn_timer,
};

use crate::states::GameState;

pub mod components;
mod resources;
mod systems;
pub mod utils;

pub const ENEMY_SPAWN_TIME: f32 = 2.0;
const SLIME_GREEN_SINGLE_SPRITE_REL_PATH: &str = "assets/sprites/slime_green_single.png";

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(
                Update,
                (
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                    handle_collision_event,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), despawn_enemies);
    }
}
