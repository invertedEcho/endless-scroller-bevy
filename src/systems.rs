use bevy::{prelude::*, window::WindowResized};

use crate::components::RelevantForMoveYOnResize;

use crate::resources::WindowDimensions;
use crate::scrolling_background::components::ScrollingBackground;
use crate::utils::get_y_of_ground;

pub fn on_resize_system(
    mut resizer_read: EventReader<WindowResized>,
    mut window_dimensions: ResMut<WindowDimensions>,
    query: Query<&mut Transform, With<RelevantForMoveYOnResize>>,
    background_query: Query<(&mut ScrollingBackground, &mut Sprite), With<ScrollingBackground>>,
) {
    let maybe_resized_event = resizer_read.read().next();

    if maybe_resized_event.is_some() {
        let window_resized_event = maybe_resized_event.unwrap();
        let new_window_height = window_resized_event.height;
        window_dimensions.width = window_resized_event.width;
        window_dimensions.height = new_window_height;

        let y_of_ground = get_y_of_ground(new_window_height);

        // If we get a window_resized_event, move y coordinate of player, ground collider
        for mut transform in query {
            transform.translation.y = y_of_ground;
        }

        // also, need to rescale background image so it fills entire screen
        for (mut bg, mut sprite) in background_query {
            let image_height = bg.height;
            let image_width = bg.width;
            let scale = new_window_height / image_height;
            let scaled_image_width = image_width * scale;

            bg.height = new_window_height;
            bg.width = scaled_image_width;
            sprite.custom_size = Some(Vec2::new(scaled_image_width, window_dimensions.height));
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
