use macroquad::{prelude::*, rand::gen_range};

pub fn random_screen_position() -> Vec2 {
    vec2(
        gen_range(0.0, screen_width()),
        gen_range(0.0, screen_height()),
    )
}
