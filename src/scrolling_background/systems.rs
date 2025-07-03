use bevy::prelude::*;

use crate::{
    obstacle::components::Platform,
    resources::WindowDimensions,
    scrolling_background::components::ScrollingBackground,
    utils::{get_left_edge_of_window, get_num_tiles},
};

const SCROLLING_SPEED: f32 = 200.0;

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

    let num_tiles = get_num_tiles(window_width, scaled_image_width);

    let left_edge_of_window = get_left_edge_of_window(window_width);

    let first_image_translation_x = left_edge_of_window + (scaled_image_width / 2.0);

    // TODO: Generate depending on num_tiles
    // let debug_colors: Vec<Color> = vec![
    //     Color::hsl(216.0, 1.0, 0.5),
    //     Color::hsl(0.0, 1.0, 0.5),
    //     Color::hsl(120.0, 1.0, 0.5),
    //     Color::hsl(60.0, 1.0, 0.5),
    // ];

    for index in 0..num_tiles {
        let translation_x = first_image_translation_x + (scaled_image_width * index as f32);
        commands.spawn((
            Sprite {
                image: asset_server.load(BACKGROUND_IMAGE_PATH),
                custom_size: Some(Vec2::new(scaled_image_width, window_height)),
                image_mode: SpriteImageMode::Auto,
                // tint color for debug purposes
                // color: debug_colors[index],
                ..default()
            },
            Transform {
                translation: Vec3::new(translation_x, 0.0, 0.0),
                ..default()
            },
            ScrollingBackground {
                height: window_height,
                width: scaled_image_width,
            },
        ));
    }
}

pub fn handle_scrolling_background(
    time: Res<Time>,
    mut scrolling_background_query: Query<
        (&mut Transform, &ScrollingBackground),
        (With<ScrollingBackground>, Without<Platform>),
    >,
    mut platform_query: Query<&mut Transform, (With<Platform>, Without<ScrollingBackground>)>,
    window_dimensions: Res<WindowDimensions>,
) {
    for (mut transform, bg) in scrolling_background_query.iter_mut() {
        let current_translation_x = transform.translation.x;
        let new_translation_x = current_translation_x - SCROLLING_SPEED * time.delta_secs();

        transform.translation.x = new_translation_x;

        let right_edge_of_image = transform.translation.x + bg.width / 2.0;
        let left_edge_of_window = get_left_edge_of_window(window_dimensions.width);

        if right_edge_of_image < left_edge_of_window {
            // TODO: Would need to use num_tiles, but before that we would need to actually
            // add/remove scrolling background images if num_tiles changes. Could do this in
            // on_resize_system
            // let num_tiles = get_num_tiles(window_dimensions.width, bg.width);
            transform.translation.x += bg.width * 3 as f32;
        }
    }
    for mut transform in platform_query.iter_mut() {
        let current_translation_x = transform.translation.x;
        let new_translation_x = current_translation_x - SCROLLING_SPEED * time.delta_secs();

        transform.translation.x = new_translation_x;
    }
}
