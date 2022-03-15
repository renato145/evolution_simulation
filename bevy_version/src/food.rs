use crate::{utils::random_screen_position, Energy, Speed};
use bevy::prelude::*;

const FOOD_SPAWN_TIME: f32 = 0.2;
const FOOD_SIZE: f32 = 5.0;
const FOOD_ENERGY: f32 = 10.0;
const FOOD_SPEED_FACTOR: f32 = 1.2;
const MAX_FOOD_INSTANCES: usize = 100;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoodSpawnTimer(Timer::from_seconds(FOOD_SPAWN_TIME, true)))
            .insert_resource(FoodCount(0))
            .add_system(food_spawn);
    }
}

pub struct FoodCount(pub usize);

struct FoodSpawnTimer(Timer);

#[derive(Component)]
pub struct Food;

fn food_spawn(
    mut commands: Commands,
    time: Res<Time>,
    windows: Res<Windows>,
    mut timer: ResMut<FoodSpawnTimer>,
    mut food_count: ResMut<FoodCount>,
) {
    if (food_count.0 < MAX_FOOD_INSTANCES) && timer.0.tick(time.delta()).just_finished() {
        let pos = random_screen_position(windows.get_primary().unwrap());
        let shape_bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 1.0, 0.0, 0.75),
                ..Default::default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 0.0)
                .with_scale(Vec3::new(FOOD_SIZE, FOOD_SIZE, 1.0)),
            ..Default::default()
        };

        commands
            .spawn_bundle(shape_bundle)
            .insert(Food)
            .insert(Energy(FOOD_ENERGY))
            .insert(Speed::random_direction(FOOD_SPEED_FACTOR));
        food_count.0 += 1;
    }
}
