use bevy::prelude::*;
use bevy_rapier2d::{prelude::Collider, render::ColliderDebugColor};

use crate::{
    resources::WindowDimensions, scrolling_background::components::ScrollingBackground,
    utils::get_num_tiles,
};

const SCROLLING_SPEED: f32 = 100.0;

const BACKGROUND_IMAGE_PATH: &str = "sprites/background.png";

pub fn spawn_scrolling_backgrounds(
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

    println!("image_width: {}", image_width);
    println!("scale: {}", scale);
    println!("scaled_image_width: {}", scaled_image_width);

    println!("window height: {}", window_height);
    println!("window width: {}", window_dimensions.width);

    let num_tiles = get_num_tiles(window_width, scaled_image_width);
    println!("num_tiles: {}", num_tiles);

    let left_edge = -window_width / 2.0;

    let first_image_translation_x = left_edge + (scaled_image_width / 2.0);

    // TODO: Generate depending on num_tiles
    let debug_colors: Vec<Hsla> = vec![
        Hsla::hsl(216.0, 1.0, 0.5),
        Hsla::hsl(0.0, 1.0, 0.5),
        Hsla::hsl(120.0, 1.0, 0.5),
        Hsla::hsl(60.0, 1.0, 0.5),
    ];

    // FIXME: This is completely wrong... Can easily be seen when using the ColliderDebugColors...
    // Overlapping
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
            ScrollingBackground {
                height: window_height,
                width: scaled_image_width,
            },
            Collider::cuboid(scaled_image_width / 2.0, window_height / 2.0),
            ColliderDebugColor(debug_colors[n]),
        ));
    }
}

pub fn handle_scrolling_background(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ScrollingBackground), With<ScrollingBackground>>,
    window_dimensions: Res<WindowDimensions>,
) {
    for (mut transform, bg) in query.iter_mut() {
        let current_translation_x = transform.translation.x;
        let new_translation_x = current_translation_x - SCROLLING_SPEED * time.delta_secs();

        transform.translation.x = new_translation_x;

        let right_edge_of_image = transform.translation.x + bg.width / 2.0;
        let left_edge_window_width = -(window_dimensions.width / 2.0);

        if right_edge_of_image < left_edge_window_width {
            let num_tiles = get_num_tiles(window_dimensions.width, bg.width);
            // TODO: Needs to be replaced with num_tiles
            transform.translation.x += bg.width * (num_tiles - 1) as f32;
        }
    }
}
