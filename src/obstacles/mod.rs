use bevy::prelude::*;
use systems::spawn_obstacle;

mod systems;

pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_obstacle);
    }
}
