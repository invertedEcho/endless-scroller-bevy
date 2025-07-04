use bevy::prelude::*;
use systems::{
    despawn_dead_menu, despawn_main_menu, despawn_score_text, handle_play_button_interaction,
    spawn_dead_menu, spawn_main_menu, spawn_score_text,
};

use crate::states::GameState;

pub mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(Update, handle_play_button_interaction)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(OnEnter(GameState::Dead), spawn_dead_menu)
            .add_systems(OnExit(GameState::Dead), despawn_dead_menu)
            .add_systems(OnEnter(GameState::Playing), spawn_score_text)
            .add_systems(OnExit(GameState::Playing), despawn_score_text);
    }
}
