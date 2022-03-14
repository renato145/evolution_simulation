use crate::{utils::random_screen_position, Speed};
use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes,
};

const FOOD_SIZE: f32 = 3.0;
const FOOD_SPEED_FACTOR: f32 = 1.2;
const MAX_FOOD_INSTANCES: usize = 20;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoodSpawnTimer(Timer::from_seconds(0.5, true)))
            .insert_resource(FoodCount(0))
            .add_system(food_spawn);
    }
}

struct FoodCount(usize);

struct FoodSpawnTimer(Timer);

#[derive(Component)]
struct Food;

fn food_spawn(
    mut commands: Commands,
    time: Res<Time>,
    windows: Res<Windows>,
    mut timer: ResMut<FoodSpawnTimer>,
    mut food_count: ResMut<FoodCount>,
) {
    if (food_count.0 < MAX_FOOD_INSTANCES) && timer.0.tick(time.delta()).just_finished() {
        let pos = random_screen_position(windows.get_primary().unwrap());
        let shape = shapes::Circle {
            radius: FOOD_SIZE,
            ..Default::default()
        };
        let shape_bundle = GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::GREEN)),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        );
        commands
            .spawn_bundle(shape_bundle)
            .insert(Food)
            .insert(Speed::random_direction(FOOD_SPEED_FACTOR));
        food_count.0 += 1;
        println!("Food count: {}", food_count.0);
    }
}
