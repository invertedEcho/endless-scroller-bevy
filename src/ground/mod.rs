use bevy::prelude::*;
use systems::spawn_ground;

mod components;
mod systems;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}
