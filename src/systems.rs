use bevy::{prelude::*, window::WindowResized};

use crate::components::RelevantForMoveYOnResize;

use crate::resources::WindowDimensions;
use crate::scrolling_background::components::ScrollingBackground;
use crate::utils::get_y_of_ground;

// TODO: Split up this function into multiple
pub fn on_resize_system(
    mut window_resized_event_reader: EventReader<WindowResized>,
    mut window_dimensions: ResMut<WindowDimensions>,
    relevant_for_move_y_on_resize_query: Query<
        &mut Transform,
        (With<RelevantForMoveYOnResize>, Without<ScrollingBackground>),
    >,
    mut scrolling_background_query: Query<
        (&mut ScrollingBackground, &mut Sprite, &mut Transform),
        With<ScrollingBackground>,
    >,
) {
    let Some(window_resized_event) = window_resized_event_reader.read().next() else {
        return;
    };

    println!("\n");
    println!("\n");
    println!("--------New window dimensions!------");
    println!("Old window height: {}", window_dimensions.height);
    println!("New window height: {}", window_resized_event.height);
    println!("\n");
    println!("Old window width: {}", window_dimensions.width);
    println!("New window width: {}", window_resized_event.width);
    println!("------------------------------------");
    println!("\n");
    println!("\n");

    let new_window_height = window_resized_event.height;
    window_dimensions.width = window_resized_event.width;
    window_dimensions.height = new_window_height;

    let y_of_ground = get_y_of_ground(new_window_height);

    // If we get a window_resized_event, move y coordinate of player, ground collider
    for mut transform in relevant_for_move_y_on_resize_query {
        transform.translation.y = y_of_ground;
    }

    let scrolling_backgrounds_sorted =
        scrolling_background_query
            .iter_mut()
            .sort_by::<(&ScrollingBackground, &Sprite, &Transform)>(
                |a: &(&ScrollingBackground, &Sprite, &Transform),
                 b: &(&ScrollingBackground, &Sprite, &Transform)| {
                    return a
                        .2
                        .translation
                        .x
                        .partial_cmp(&b.2.translation.x)
                        .expect("Sorting should not panic?");
                },
            );

    // also, need to rescale background image so it fills entire screen again
    for (index, (mut bg, mut sprite, mut transform)) in scrolling_backgrounds_sorted.enumerate() {
        println!("Transform: {:?}", transform);
        let old_image_height = bg.height;
        let old_image_width = bg.width;

        let scale = new_window_height / old_image_height;
        let new_image_width_scaled = old_image_width * scale;
        println!("Scale: {}", scale);

        println!("Old image height: {}", old_image_height);
        println!("New image height: {}", new_window_height);

        println!("Old image width: {}", old_image_width);
        println!("New image width: {}", new_image_width_scaled);

        if old_image_height == new_window_height && old_image_width == new_image_width_scaled {
            println!("Nothing changed, skipping rescaling and moving scrolling background images.");
            return;
        }

        bg.height = new_window_height;
        bg.width = new_image_width_scaled;

        sprite.custom_size = Some(Vec2::new(new_image_width_scaled, new_window_height));

        // then, move all but the leftmost image so we close the gaps or avoid having overlapping
        // images.

        let difference_image_width = old_image_width - new_image_width_scaled;
        println!("Difference image width: {}", difference_image_width);

        if difference_image_width > 0.0 {
            println!("Assumed window width decrease");
            transform.translation.x -= difference_image_width * (index + 1) as f32;
        } else {
            println!("Assumed window width increase");
            transform.translation.x += difference_image_width * (index + 1) as f32;
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
