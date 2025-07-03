use bevy::{prelude::*, window::WindowResized};

use crate::components::RelevantForMoveY;
use crate::player::components::Player;
use crate::player::utils::get_player_ball_radius;
use crate::resources::WindowDimensions;
use crate::scrolling_background::components::BackgroundImage;
use crate::states::GameState;
use crate::utils::get_y_of_ground;

// TODO: Split up this function into multiple
// TODO: Move background images if in main_menu so no gap to the left exists
pub fn on_resize_system(
    mut window_resized_event_reader: EventReader<WindowResized>,
    mut window_dimensions: ResMut<WindowDimensions>,
    mut scrolling_background_query: Query<
        (&mut BackgroundImage, &mut Sprite, &mut Transform),
        (With<BackgroundImage>, Without<RelevantForMoveY>),
    >,
    mut relevant_for_move_y_query: Query<
        &mut Transform,
        (With<RelevantForMoveY>, Without<BackgroundImage>),
    >,
    mut player_query: Query<
        &mut Transform,
        (
            With<Player>,
            Without<RelevantForMoveY>,
            Without<BackgroundImage>,
        ),
    >,
    game_state: Res<State<GameState>>,
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

    for mut relevant_for_move_y in relevant_for_move_y_query.iter_mut() {
        relevant_for_move_y.translation.y = y_of_ground;
    }

    if game_state.get() == &GameState::PLAYING {
        let mut player = player_query
            .single_mut()
            .expect("Exactly one player exists");
        let player_ball_radius = get_player_ball_radius() as f32;
        player.translation.y = y_of_ground + player_ball_radius;
    }

    let scrolling_backgrounds_sorted =
        scrolling_background_query
            .iter_mut()
            .sort_by::<(&BackgroundImage, &Sprite, &Transform)>(
                |a: &(&BackgroundImage, &Sprite, &Transform),
                 b: &(&BackgroundImage, &Sprite, &Transform)| {
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
        let old_image_height = bg.height;
        let old_image_width = bg.width;

        let scale = new_window_height / old_image_height;
        let new_image_width_scaled = old_image_width * scale;

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

        if index == 0 {
            continue;
        }

        let difference_image_width = old_image_width - new_image_width_scaled;
        println!("Difference image width: {}", difference_image_width);

        let absolute_difference_with_index = (difference_image_width * index as f32).abs();
        if difference_image_width > 0.0 {
            println!("\n");
            println!("\n");
            println!("Assumed window width decrease, decreasing translation.x");
            println!("Current translation.x: {}", transform.translation.x);
            println!("Decreasing by: {}", absolute_difference_with_index);
            println!("\n");
            println!("\n");
            transform.translation.x -= absolute_difference_with_index;
        } else {
            println!("\n");
            println!("\n");
            println!("Assumed window width increase, increasing translation.x");
            println!("Current translation.x: {}", transform.translation.x);
            println!("Increasing by: {}", absolute_difference_with_index);
            println!("\n");
            println!("\n");
            transform.translation.x += absolute_difference_with_index;
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
