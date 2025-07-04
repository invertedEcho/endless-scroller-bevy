use crate::states::GameState;

const MAIN_MENU_SCROLLING_SPEED: f32 = 50.0;
const PLAYING_SCROLLING_SPEED: f32 = 200.0;
const DEAD_SCROLLING_SPEED: f32 = 0.0;

pub fn get_scrolling_speed(game_state: &GameState) -> f32 {
    match game_state {
        GameState::MainMenu => MAIN_MENU_SCROLLING_SPEED,
        GameState::Dead => DEAD_SCROLLING_SPEED,
        GameState::Playing => PLAYING_SCROLLING_SPEED,
    }
}
