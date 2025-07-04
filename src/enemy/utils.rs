use imagesize::ImageSize;

use super::SLIME_GREEN_SINGLE_SPRITE_REL_PATH;

pub fn get_image_size_enemy_sprite() -> ImageSize {
    imagesize::size(SLIME_GREEN_SINGLE_SPRITE_REL_PATH)
        .expect("Can get imagesize of single platform sprite")
}

