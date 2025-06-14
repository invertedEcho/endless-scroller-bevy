use bevy::{prelude::*, window::WindowResized};

use crate::{RedrawBackgroundEvent, WindowDimensions, components::ScrollingBackground};

const SCROLLING_SPEED: f32 = 100.0;

const BACKGROUND_LAYERS: [&str; 12] = [
    "Layer_0000_9.png",
    "Layer_0001_8.png",
    "Layer_0002_7.png",
    "Layer_0003_6.png",
    "Layer_0004_Lights.png",
    "Layer_0005_5.png",
    "Layer_0006_4.png",
    "Layer_0007_Lights.png",
    "Layer_0008_3.png",
    "Layer_0009_2.png",
    "Layer_0010_1.png",
    "Layer_0011_0.png",
];

pub fn spawn_entire_background_image_from_layers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_dimensions: Res<WindowDimensions>,
) {
    let window_height = window_dimensions.height;

    for n in 0..BACKGROUND_LAYERS.len() {
        let reverse_index = BACKGROUND_LAYERS.len() - n - 1;
        println!("{}", reverse_index);

        let background_layer = BACKGROUND_LAYERS[reverse_index];
        let relative_path_for_background_layer = format!("assets/sprites/{}", background_layer);

        println!("{}", relative_path_for_background_layer);

        let image_size =
            imagesize::size(relative_path_for_background_layer).expect("Can get image size");

        let image_width = image_size.width as f32;
        let image_height = image_size.height as f32;

        println!("image_width: {}", image_width);
        println!("window height: {}", window_height);

        let scale = window_height / image_height;
        let scaled_image_width = image_width * scale;

        commands.spawn((
            Sprite {
                image: asset_server.load(format!("sprites/{}", background_layer)),
                // TODO: How can we get physical height of window? Or how can i properly scale this
                // to fill entire window height
                custom_size: Some(Vec2::new(scaled_image_width, window_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform {
                translation: Vec3::new(-scaled_image_width, 0.0, n as f32),
                ..default()
            },
            ScrollingBackground {
                width: scaled_image_width,
            },
        ));

        commands.spawn((
            Sprite {
                image: asset_server.load(format!("sprites/{}", background_layer)),
                custom_size: Some(Vec2::new(scaled_image_width, window_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform {
                translation: Vec3::new(0.0, 0.0, n as f32),
                ..default()
            },
            ScrollingBackground {
                width: scaled_image_width,
            },
        ));

        commands.spawn((
            Sprite {
                image: asset_server.load(format!("sprites/{}", background_layer)),
                custom_size: Some(Vec2::new(scaled_image_width, window_height)),
                image_mode: SpriteImageMode::Auto,
                ..default()
            },
            Transform {
                translation: Vec3::new(scaled_image_width, 0.0, n as f32),
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
        transform.translation.x -= SCROLLING_SPEED * time.delta_secs();

        if transform.translation.x < -bg.width {
            // Because we have three tiles
            transform.translation.x += bg.width * 3.0
        }
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
