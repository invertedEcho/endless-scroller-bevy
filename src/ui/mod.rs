use bevy::prelude::*;
use systems::spawn_main_menu;

mod systems;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_main_menu);
    }
}
