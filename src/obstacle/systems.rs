use bevy::prelude::*;

use crate::{resources::WindowDimensions, utils::get_y_of_ground};

pub fn random_spawn_obstacle(window_dimensions: Res<WindowDimensions>) {
    let y_of_ground = get_y_of_ground(window_dimensions.height);
}
