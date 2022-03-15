use crate::{utils::random_screen_position, Speed};
use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes,
};

const SLIME_SPAWN_TIME: f32 = 1.5;
const SLIME_SPEED_FACTOR: f32 = 1.8;

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SlimeSpawnTimer(Timer::from_seconds(SLIME_SPAWN_TIME, true)))
            .insert_resource(SlimeCount(0))
            .add_system(slime_spawn);
    }
}

struct SlimeCount(usize);

struct SlimeSpawnTimer(Timer);

#[derive(Component)]
struct Slime;

fn slime_spawn(
    mut commands: Commands,
    time: Res<Time>,
    windows: Res<Windows>,
    mut timer: ResMut<SlimeSpawnTimer>,
    mut food_count: ResMut<SlimeCount>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let pos = random_screen_position(windows.get_primary().unwrap());
        let shape = shapes::Circle {
            radius: 7.5,
            ..Default::default()
        };
        let shape_bundle = GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::RED)),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        );
        commands
            .spawn_bundle(shape_bundle)
            .insert(Slime)
            .insert(Speed::random_direction(SLIME_SPEED_FACTOR));
        food_count.0 += 1;
    }
}
