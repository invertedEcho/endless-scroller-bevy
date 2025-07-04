use bevy::prelude::*;

#[derive(Resource)]
pub struct ScoreResource {
    pub count: u32,
}

impl Default for ScoreResource {
    fn default() -> Self {
        ScoreResource { count: 0 }
    }
}
