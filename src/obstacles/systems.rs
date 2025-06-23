use bevy::prelude::*;
use rand::{self, Rng};

use crate::{resources::WindowDimensions, utils::get_y_of_ground};

pub fn spawn_obstacle(
    mut commands: Commands,
    window_dimensions: Res<WindowDimensions>,
    asset_server: Res<AssetServer>,
) {
    // 0.04 % to spawn an obstacle
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(..=25);

    if random_number != 10 {
        return;
    }

    let window_height = window_dimensions.height;

    let y_of_ground = get_y_of_ground(window_height);

    let x_offset_pixel = 100.0;

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/small_platform_green.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(x_offset_pixel, y_of_ground, 0.0),
            ..default()
        },
    ));
}
