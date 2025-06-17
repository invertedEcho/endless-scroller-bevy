use bevy::{prelude::*, window::WindowResized};

use crate::{
    RedrawBackgroundEvent, RedrawKnightEvent, WindowDimensions,
    components::{Knight, RelevantForDespawnOnResize, ScrollingBackground},
};

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
            RelevantForDespawnOnResize {},
        ));
    }
}

pub fn spawn_knight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let window_height = window_dimensions.height;

    // TODO: I have no idea what to call these variables
    let bottom_y = -(window_height / 2.0);

    let ten_percent_of_window_height = bottom_y / 5.0;
    let one_percent_of_window_height = ten_percent_of_window_height / 10.0;

    // This is at 90.5 percent of the top of the image, but center is at 0x0, so we dont multiply
    // by 90.5, but need to substract 50.0
    let desired_thing = one_percent_of_window_height * 40.5;

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/knight_single.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, desired_thing, 1.0),
            ..default()
        },
        Knight {},
        RelevantForDespawnOnResize {},
    ));
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
            // TODO: Needs to be replaced with num_tiles
            transform.translation.x += bg.width * 3.0;
        }
    }
}

pub fn on_resize_system(
    mut commands: Commands,
    mut resizer_read: EventReader<WindowResized>,
    mut background_redraw_event_writer: EventWriter<RedrawBackgroundEvent>,
    mut knight_redraw_event_writer: EventWriter<RedrawKnightEvent>,
    entities_to_despawn: Query<Entity, With<RelevantForDespawnOnResize>>,
    mut window_dimensions: ResMut<WindowDimensions>,
) {
    for window_resized_event in resizer_read.read() {
        for entity in entities_to_despawn {
            println!("Despawning entity: {:?}", entity);
            commands.entity(entity).despawn();
        }

        window_dimensions.width = window_resized_event.width;
        window_dimensions.height = window_resized_event.height;

        background_redraw_event_writer.write(RedrawBackgroundEvent);
        knight_redraw_event_writer.write(RedrawKnightEvent);
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

pub fn redraw_knight_system(
    mut event_reader: EventReader<RedrawKnightEvent>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    if event_reader.read().next().is_some() {
        spawn_knight(commands, asset_server, window_dimensions);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
