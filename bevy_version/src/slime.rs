use crate::{
    food::{Food, FoodCount},
    utils::random_screen_position,
    Speed,
};
use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes,
};

const SLIME_SPAWN_TIME: f32 = 1.5;
const SLIME_SPEED_FACTOR: f32 = 1.8;
const SLIME_INITIAL_SIZE: f32 = 10.0;

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SlimeSpawnTimer(Timer::from_seconds(SLIME_SPAWN_TIME, true)))
            .insert_resource(SlimeCount(0))
            .add_system(slime_spawn)
            .add_system(slime_eat);
    }
}

pub struct SlimeCount(pub usize);
struct SlimeSpawnTimer(Timer);

#[derive(Component)]
struct Slime;

#[derive(Component)]
enum SlimeState {
    Normal,
    Eating,
}

// TODO: final game should not spawn slimes
fn slime_spawn(
    mut commands: Commands,
    time: Res<Time>,
    windows: Res<Windows>,
    mut timer: ResMut<SlimeSpawnTimer>,
    mut slime_count: ResMut<SlimeCount>,
) {
    if (slime_count.0 <= 5) && timer.0.tick(time.delta()).just_finished() {
        let pos = random_screen_position(windows.get_primary().unwrap());
        let shape = shapes::Circle {
            ..Default::default()
        };
        let shape_bundle = GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::RED)),
            Transform::from_xyz(pos.x, pos.y, 0.0).with_scale(Vec3::new(
                SLIME_INITIAL_SIZE,
                SLIME_INITIAL_SIZE,
                1.0,
            )),
        );
        commands
            .spawn_bundle(shape_bundle)
            .insert(Slime)
            .insert(Speed::random_direction(SLIME_SPEED_FACTOR))
            .insert(SlimeState::Normal);
        slime_count.0 += 1;
    }
}

fn slime_eat(
    mut commands: Commands,
    mut food_count: ResMut<FoodCount>,
    mut slime_query: Query<(&mut SlimeState, &Transform), With<Slime>>,
    food_query: Query<(Entity, &Transform), With<Food>>,
) {
    for (mut slime_state, slime_tf) in slime_query.iter_mut() {
        *slime_state = SlimeState::Normal;
        let slime_sz = slime_tf.scale.abs().xy();
        for (food_entity, food_tf) in food_query.iter() {
            let food_sz = food_tf.scale.abs().xy();
            let collision = collide(slime_tf.translation, slime_sz, food_tf.translation, food_sz);
            if let Some(_) = collision {
                *slime_state = SlimeState::Eating;
                commands.entity(food_entity).despawn();
                food_count.0 -= 1;
            }
        }
    }
}

// fn update_slime_color(mut query: Query<&mut DrawMode, With<Slime>>) {
//     for draw_mode in query.iter_mut() {
//         if let DrawMode::Fill(fill) = draw_mode.as_mut() {
//             fill.color = Color::PINK;
//         }
//     }
// }
