use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    components::{RelevantForMoveX, RelevantForMoveY},
    resources::WindowDimensions,
    utils::get_y_of_ground,
};

pub fn spawn_ground_collidier(mut commands: Commands, window_dimensions: Res<WindowDimensions>) {
    let y_of_ground = get_y_of_ground(window_dimensions.height);

    println!("Spawning ground_collider at x:0 y: {:?}", y_of_ground);

    commands
        .spawn((
            Collider::cuboid(window_dimensions.width / 2.0, 0.0),
            RelevantForMoveY,
            RelevantForMoveX,
        ))
        .insert(Transform::from_xyz(0.0, y_of_ground, 0.0));
}
