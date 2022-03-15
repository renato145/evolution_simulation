use self::{food::FoodPlugin, slime::SlimePlugin};
use crate::utils::{polar_to_cartesian, wrap_around};
use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

pub mod food;
pub mod slime;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FoodPlugin)
            .add_plugin(SlimePlugin)
            .add_system(entity_move);
    }
}

#[derive(Component)]
struct Speed {
    speed_factor: f32,
    speed: Vec2,
}

impl Speed {
    fn random_direction(speed_factor: f32) -> Self {
        let mut rnd = rand::thread_rng();
        let speed = polar_to_cartesian(speed_factor, rnd.gen_range(0.0..=PI * 2.0));
        Speed {
            speed_factor,
            speed,
        }
    }

    fn modify_direction(&mut self, direction: f32) {
        self.speed = polar_to_cartesian(self.speed_factor, direction);
    }
}

fn entity_move(windows: Res<Windows>, mut query: Query<(&mut Transform, &Speed)>) {
    let window = windows.get_primary().unwrap();
    let (w_2, h_2) = (window.width() / 2.0, window.height() / 2.0);
    for (mut tf, speed) in query.iter_mut() {
        tf.translation.x += speed.speed.x;
        tf.translation.y += speed.speed.y;
        wrap_around(&mut tf, w_2, h_2);
    }
}

#[derive(Component)]
struct Energy(f32);
