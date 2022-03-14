#![allow(unused)]
use bevy::{math::Vec2, prelude::Transform, window::Window};
use rand::Rng;

pub fn random_screen_position(window: &Window) -> Vec2 {
    let mut rnd = rand::thread_rng();
    let (w_2, h_2) = (window.width() / 2.0, window.height() / 2.0);
    let pos = (rnd.gen_range(-w_2..=w_2), rnd.gen_range(-h_2..=h_2));
    Vec2::new(pos.0, pos.1)
}

/// Converts 2d polar coordinates to 2d cartesian coordinates.
pub fn polar_to_cartesian(rho: f32, theta: f32) -> Vec2 {
    Vec2::new(rho * theta.cos(), rho * theta.sin())
}

/// Converts 2d cartesian coordinates to 2d polar coordinates.
pub fn cartesian_to_polar(cartesian: Vec2) -> Vec2 {
    Vec2::new(
        (cartesian.x.powi(2) + cartesian.y.powi(2)).sqrt(),
        cartesian.y.atan2(cartesian.x),
    )
}

/// Wraps a positions offsets around the screen
pub fn wrap_around(transform: &mut Transform, w_2: f32, h_2: f32) {
    if transform.translation.x > w_2 {
        transform.translation.x = -w_2;
    } else if transform.translation.x < -w_2 {
        transform.translation.x = w_2;
    }
    if transform.translation.y > h_2 {
        transform.translation.y = -h_2;
    } else if transform.translation.y < -h_2 {
        transform.translation.y = h_2;
    }
}
