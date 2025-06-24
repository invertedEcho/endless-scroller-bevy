use bevy::{prelude::*, window::WindowResized};

use crate::components::RelevantForMoveYOnResize;

use crate::resources::WindowDimensions;
use crate::scrolling_background::components::ScrollingBackground;
use crate::utils::get_y_of_ground;

pub fn on_resize_system(
    mut resizer_read: EventReader<WindowResized>,
    mut window_dimensions: ResMut<WindowDimensions>,
    query: Query<&mut Transform, (With<RelevantForMoveYOnResize>, Without<ScrollingBackground>)>,
    mut scrolling_background_query: Query<
        (&mut ScrollingBackground, &mut Sprite, &mut Transform),
        With<ScrollingBackground>,
    >,
) {
    let Some(event) = resizer_read.read().next() else {
        return;
    };

    let new_window_height = event.height;
    window_dimensions.width = event.width;
    window_dimensions.height = new_window_height;

    let y_of_ground = get_y_of_ground(new_window_height);

    // If we get a window_resized_event, move y coordinate of player, ground collider
    for mut transform in query {
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

    let mut old_image_width = 0.0;
    let mut new_image_width_scaled = 0.0;

    // also, need to rescale background image so it fills entire screen again
    for (mut bg, mut sprite, transform) in scrolling_backgrounds_sorted {
        println!("Transform: {:?}", transform);
        let old_image_height = bg.height;
        old_image_width = bg.width;

        let scale = new_window_height / old_image_height;
        new_image_width_scaled = old_image_width * scale;
        println!("Scale: {}", scale);

        println!("Old image height: {}", old_image_height);
        println!("New image height: {}", new_window_height);

        println!("Old image width: {}", old_image_width);
        println!("New image width: {}", new_image_width_scaled);

        bg.height = new_window_height;
        bg.width = new_image_width_scaled;

        sprite.custom_size = Some(Vec2::new(new_image_width_scaled, new_window_height));
    }

    // then, move all but the leftmost image so we close the gaps or avoid having overlapping
    // images.
    let difference_image_width = old_image_width - new_image_width_scaled;
    println!("Difference image width");
    for (i, (_, _, mut transform)) in scrolling_background_query.iter_mut().enumerate().skip(1) {
        if difference_image_width > 0.0 {
            println!("Assumed window width decrease");
            transform.translation.x -= difference_image_width * (i + 1) as f32;
        } else {
            println!("Assumed window width increase");
            transform.translation.x += difference_image_width * (i + 1) as f32;
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
