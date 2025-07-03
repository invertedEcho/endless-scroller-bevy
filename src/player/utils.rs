use super::PLAYER_SIZE_FACTOR;

pub struct ImageSizePlayerSprite {
    pub width: f32,
    pub height: f32,
}

pub fn get_image_size_player_sprite() -> ImageSizePlayerSprite {
    let image_size_player_sprite = imagesize::size("assets/sprites/knight_single.png")
        .expect("Can get image size of player sprite");
    return ImageSizePlayerSprite {
        width: image_size_player_sprite.width as f32 * PLAYER_SIZE_FACTOR,
        height: image_size_player_sprite.height as f32 * PLAYER_SIZE_FACTOR,
    };
}

pub fn get_player_ball_radius() -> f32 {
    get_image_size_player_sprite().height / 2.0
}
