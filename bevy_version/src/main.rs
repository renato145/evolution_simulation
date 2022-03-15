use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use food::FoodPlugin;
use rand::Rng;
use slime::SlimePlugin;
use std::f32::consts::PI;
use utils::{polar_to_cartesian, wrap_around};

mod food;
mod slime;
mod utils;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Evolution simulation".to_string(),
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(FoodPlugin)
        .add_plugin(SlimePlugin)
        .add_system(entity_move)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

#[derive(Component)]
struct Speed(Vec2);

impl Speed {
    fn random_direction(speed_factor: f32) -> Self {
        let mut rnd = rand::thread_rng();
        let direction = rnd.gen_range(0.0..=PI * 2.0);
        Speed(polar_to_cartesian(speed_factor, direction))
    }
}

fn entity_move(windows: Res<Windows>, mut query: Query<(&mut Transform, &Speed)>) {
    let window = windows.get_primary().unwrap();
    let (w_2, h_2) = (window.width() / 2.0, window.height() / 2.0);
    for (mut tf, speed) in query.iter_mut() {
        tf.translation.x += speed.0.x;
        tf.translation.y += speed.0.y;
        wrap_around(&mut tf, w_2, h_2);
    }
}
