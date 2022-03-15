use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use food::{FoodCount, FoodPlugin};
use rand::Rng;
use slime::{SlimeCount, SlimePlugin};
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
        .add_system(show_stats)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Show stats
    let style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Slimes: ".to_string(),
                    style: style.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style: style.clone(),
                },
                TextSection {
                    value: "\nFoods: ".to_string(),
                    style: style.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style,
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
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

fn show_stats(
    slime_count: Res<SlimeCount>,
    food_count: Res<FoodCount>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[1].value = format!("{}", slime_count.0);
    text.sections[3].value = format!("{}", food_count.0);
}
