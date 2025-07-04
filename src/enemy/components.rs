use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub past_player: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy { past_player: false }
    }
}
