use macroquad::{prelude::*, rand::gen_range};

pub fn random_screen_position() -> Vec2 {
    vec2(
        gen_range(0.0, screen_width()),
        gen_range(0.0, screen_height()),
    )
}

/// Wraps a positions offsets around the screen
pub fn wrap_around(pos: &Vec2) -> Vec2 {
    let mut new_pos = Vec2::new(pos.x, pos.y);
    if new_pos.x > screen_width() {
        new_pos.x = 0.;
    }
    if new_pos.x < 0. {
        new_pos.x = screen_width()
    }
    if new_pos.y > screen_height() {
        new_pos.y = 0.;
    }
    if new_pos.y < 0. {
        new_pos.y = screen_height()
    }
    new_pos
}

/// Get angle direction from point a to b
pub fn get_angle_direction(a: Vec2, b: Vec2) -> f32 {
    let diff = b - a;
    diff.y.atan2(diff.x)
}
