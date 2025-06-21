use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use events::RedrawGroundColliderEvent;
use systems::{redraw_ground_collider_system, spawn_ground_collidier};

pub mod events;
mod systems;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, spawn_ground_collidier)
            .add_systems(Update, redraw_ground_collider_system)
            .add_event::<RedrawGroundColliderEvent>();
    }
}
