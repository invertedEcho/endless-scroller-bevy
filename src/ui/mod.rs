use bevy::prelude::*;
use systems::{
    despawn_dead_menu, despawn_main_menu, handle_play_button_interaction, spawn_dead_menu,
    spawn_main_menu,
};

use crate::states::GameState;

mod components;
mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu)
            .add_systems(Update, handle_play_button_interaction)
            .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
            .add_systems(OnEnter(GameState::Dead), spawn_dead_menu)
            .add_systems(OnExit(GameState::Dead), despawn_dead_menu);
    }
}
