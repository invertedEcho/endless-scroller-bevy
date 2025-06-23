use bevy::prelude::*;

use crate::{background_image::components::BackgroundImage, resources::WindowDimensions};

use super::events::RedrawScrollingBackgroundEvent;

const BACKGROUND_IMAGE_PATH: &str = "sprites/background.png";

pub fn spawn_first_background_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let window_height = window_dimensions.height;
    let window_width = window_dimensions.width;

    let image_size = imagesize::size(format!("assets/{}", BACKGROUND_IMAGE_PATH))
        .expect("Can get image_size for BACKGROUND_IMAGE");
    let image_width = image_size.width as f32;
    let image_height = image_size.height as f32;

    let scale = window_height / image_height;
    let scaled_image_width = image_width * scale;

    let num_tiles = (window_width / scaled_image_width).ceil() as usize + 1;
    println!("Spawning first background tiles, num_tiles: {}", num_tiles);

    let left_edge = -window_width / 2.0;

    let first_image_translation_x = left_edge + (scaled_image_width / 2.0);

    for n in 0..num_tiles {
        let x = first_image_translation_x + (scaled_image_width * n as f32);
        commands.spawn((
            Sprite {
                image: asset_server.load(BACKGROUND_IMAGE_PATH),
                custom_size: Some(Vec2::new(scaled_image_width, window_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform {
                translation: Vec3::new(x, 0.0, 0.0),
                ..default()
            },
            BackgroundImage {
                width: scaled_image_width,
            },
        ));
    }
}

pub fn redraw_background_system(
    mut event_reader: EventReader<RedrawScrollingBackgroundEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    if event_reader.read().next().is_some() {
        spawn_first_background_tiles(commands, asset_server, window_dimensions);
    }
}

// pub fn handle_scrolling_background(
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, &ScrollingBackground), With<ScrollingBackground>>,
//     window_dimensions: Res<WindowDimensions>,
// ) {
//     for (mut transform, bg) in query.iter_mut() {
//         let current_translation_x = transform.translation.x;
//         let new_translation_x = current_translation_x - SCROLLING_SPEED * time.delta_secs();
//
//         transform.translation.x = new_translation_x;
//
//         let right_edge_of_image = transform.translation.x + bg.width / 2.0;
//         let left_edge_window_width = -(window_dimensions.width / 2.0);
//
//         if right_edge_of_image < left_edge_window_width {
//             // TODO: Needs to be replaced with num_tiles
//             transform.translation.x += bg.width * 3.0;
//         }
//     }
// }
//
