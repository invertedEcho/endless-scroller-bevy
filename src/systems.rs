use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::ScrollingBackground;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single().expect("Can get primary window");
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/background.png"),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        ScrollingBackground {},
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("sprites/background.png"),
            custom_size: Some(Vec2::new(window.width(), window.height())),
            image_mode: SpriteImageMode::Auto,
            ..default()
        },
        Transform {
            translation: Vec3::new(window.width(), 0.0, 0.0),
            ..default()
        },
        ScrollingBackground {},
    ));
}

pub fn handle_scrolling_background(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<ScrollingBackground>>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= 80.0 * time.delta_secs();

        if transform.translation.x < -3000.0 {
            transform.translation.x += 3000.0 * 2.0;
        }
    }
}
