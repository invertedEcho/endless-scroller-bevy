use bevy::{prelude::*, window::WindowResized};

use crate::{RedrawBackgroundEvent, WindowDimensions, components::ScrollingBackground};

const SCROLLING_SPEED: f32 = 100.0;

const BACKGROUND_IMAGE_PATH: &str = "sprites/background.png";

pub fn spawn_entire_background_image_from_layers(
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

    let num_tiles = (window_width / scaled_image_width).ceil() as usize + 1;
    println!("num_tiles: {}", num_tiles);

    let left_edge = -window_width / 2.0;

    // window_width is 1000
    // left edge would be -500

    // -500 + 800 / 2.0 = -100 which makes sense when drawing this on paper
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
            ScrollingBackground {
                width: scaled_image_width,
            },
        ));
    }
}

pub fn spawn_knight(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/knight_single.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 20.0),
            ..default()
        },
    ));
}

pub fn handle_scrolling_background(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &ScrollingBackground), With<ScrollingBackground>>,
) {
    for (mut transform, bg) in query.iter_mut() {
        let current_translation_x = transform.translation.x;
        let new_translation_x = current_translation_x - SCROLLING_SPEED * time.delta_secs();

        transform.translation.x = new_translation_x;
    }
}

pub fn on_resize_system(
    mut commands: Commands,
    mut resizer_read: EventReader<WindowResized>,
    mut redraw_writer: EventWriter<RedrawBackgroundEvent>,
    scrolling_background_query: Query<Entity, With<ScrollingBackground>>,
    mut window_dimensions: ResMut<WindowDimensions>,
) {
    for window_resized_event in resizer_read.read() {
        for entity in scrolling_background_query {
            commands.entity(entity).despawn();
        }

        window_dimensions.width = window_resized_event.width;
        window_dimensions.height = window_resized_event.height;

        redraw_writer.write(RedrawBackgroundEvent);
    }
}

pub fn redraw_background_system(
    mut event_reader: EventReader<RedrawBackgroundEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    if event_reader.read().next().is_some() {
        spawn_entire_background_image_from_layers(commands, asset_server, window_dimensions);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
