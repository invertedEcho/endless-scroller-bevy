use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{components::RelevantForMoveY, resources::WindowDimensions, utils::get_y_of_ground};

use super::components::GroundCollider;

pub fn spawn_ground_collidier(mut commands: Commands, window_dimensions: Res<WindowDimensions>) {
    let y_of_ground = get_y_of_ground(window_dimensions.height);

    commands.spawn((
        Collider::cuboid(window_dimensions.width / 2.0, 0.0),
        Transform::from_xyz(0.0, y_of_ground, 0.0),
        GroundCollider,
        RelevantForMoveY,
    ));
}
