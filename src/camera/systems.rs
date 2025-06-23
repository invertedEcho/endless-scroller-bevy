use bevy::prelude::*;

use crate::components::RelevantForMoveX;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), RelevantForMoveX));
}
