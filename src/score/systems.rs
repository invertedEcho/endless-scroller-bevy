use bevy::prelude::*;

use crate::{
    enemy::{components::Enemy, utils::get_image_size_enemy_sprite},
    player::{components::Player, utils::get_player_ball_radius},
    ui::components::ScoreText,
};

use super::resources::ScoreResource;

pub fn check_if_player_beyond_enemy(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Enemy, &Transform), With<Enemy>>,
    mut score_resource: ResMut<ScoreResource>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    let player = player_query.single().expect("Exactly one player exists");

    for (mut enemy, transform) in enemy_query.iter_mut() {
        if enemy.past_player {
            continue;
        }

        let player_collider_ball_radius = get_player_ball_radius() as f32;
        let left_edge_of_player = player.translation.x - player_collider_ball_radius;

        let image_size_enemy_sprite = get_image_size_enemy_sprite();
        let right_edge_of_enemy =
            transform.translation.x + (image_size_enemy_sprite.width / 2) as f32;

        if right_edge_of_enemy < left_edge_of_player {
            enemy.past_player = true;
            score_resource.count += 1;

            let mut score_text = score_text_query
                .single_mut()
                .expect("Exactly one score_text exists");
            *score_text = Text::new(format!("Score: {}", score_resource.count));
            println!("Detected player beyond enemy, increased score.");
        }
    }
}

pub fn reset_score_resource(mut score_resource: ResMut<ScoreResource>) {
    score_resource.count = 0;
}
