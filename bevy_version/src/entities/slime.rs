use super::{
    food::{Food, FoodCount},
    Energy, Speed,
};
use crate::{
    interactions::MousePosition,
    utils::{get_angle_direction, random_screen_position},
};
use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes,
};
use std::collections::{HashMap, HashSet};

const SLIME_SPAWN_TIME: f32 = 2.5;
const SLIME_SPEED_FACTOR: f32 = 2.8;
const SLIME_INITIAL_SIZE: f32 = 5.0;
const SLIME_MAX_SIZE: f32 = 100.0;
const SLIME_INITIAL_ENERGY: f32 = 30.0;
const SLIME_VISION_RANGE: f32 = 45.0;
const SLIME_STEP_COST: f32 = 0.1;

pub struct SlimePlugin;

impl Plugin for SlimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SlimeSpawnTimer(Timer::from_seconds(SLIME_SPAWN_TIME, true)))
            .insert_resource(SlimeCount(0))
            .insert_resource(HoveredSlimes(HashMap::new()))
            .add_system(slime_spawn)
            .add_system(slime_life_cost)
            .add_system(slime_follow_food)
            .add_system(slime_eat)
            .add_system(slime_update_draw)
            .add_system(hover_actions);
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
    if (slime_count.0 < 30) && timer.0.tick(time.delta()).just_finished() {
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
            .insert(Energy(SLIME_INITIAL_ENERGY))
            .insert(Speed::random_direction(SLIME_SPEED_FACTOR))
            .insert(SlimeState::Normal);
        slime_count.0 += 1;
    }
}

fn slime_life_cost(mut commands: Commands, mut query: Query<(Entity, &mut Energy), With<Slime>>) {
    for (entity, mut energy) in query.iter_mut() {
        energy.0 -= SLIME_STEP_COST;
        if energy.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn slime_follow_food(
    mut slime_query: Query<(&mut Speed, &Transform), With<Slime>>,
    food_query: Query<&Transform, With<Food>>,
) {
    for (mut speed, slime_tf) in slime_query.iter_mut() {
        let slime_pos = slime_tf.translation.xy();
        // Get closest food position
        if let Some((food_pos, distance)) = food_query
            .iter()
            .map(|food_tf| {
                let food_pos = food_tf.translation.xy();
                (food_pos, slime_pos.distance(food_pos))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        {
            let slime_size = slime_tf.scale.x;
            if distance <= (SLIME_VISION_RANGE + slime_size) {
                let direction = get_angle_direction(slime_pos, food_pos);
                speed.modify_direction(direction);
            }
        }
    }
}

fn slime_eat(
    mut commands: Commands,
    mut food_count: ResMut<FoodCount>,
    mut slime_query: Query<(&mut Energy, &mut SlimeState, &Transform), With<Slime>>,
    food_query: Query<(Entity, &Energy, &Transform), (With<Food>, Without<Slime>)>,
) {
    let mut eaten_foods: HashSet<Entity> = HashSet::new();
    for (mut slime_energy, mut slime_state, slime_tf) in slime_query.iter_mut() {
        *slime_state = SlimeState::Normal;
        let slime_sz = slime_tf.scale.abs().xy();
        for (food_entity, food_energy, food_tf) in food_query.iter() {
            // Skip already eaten foods
            if eaten_foods.get(&food_entity).is_some() {
                continue;
            }
            let food_sz = food_tf.scale.abs().xy();
            let collision = collide(slime_tf.translation, slime_sz, food_tf.translation, food_sz);
            if collision.is_some() {
                *slime_state = SlimeState::Eating;
                slime_energy.0 += food_energy.0;
                commands.entity(food_entity).despawn();
                food_count.0 -= 1;
                eaten_foods.insert(food_entity);
            }
        }
    }
}

fn slime_update_draw(mut query: Query<(&Energy, &mut Transform), With<Slime>>) {
    for (energy, mut tf) in query.iter_mut() {
        // Update size
        let size = (SLIME_INITIAL_SIZE + energy.0 / 10.0).min(SLIME_MAX_SIZE);
        tf.scale.x = size;
        tf.scale.y = size;
    }
}

// fn slime_update_draw2(
//     mut query: Query<(&Energy, &SlimeState, &mut Transform, &mut DrawMode, &mut Sprite), With<Slime>>,
// ) {
//     for (energy, state, mut tf, mut draw_mode, mut sprite) in query.iter_mut() {
//         sprite.color
//         // Update size
//         let size = (SLIME_INITIAL_SIZE + energy.0 / 50.0).min(SLIME_MAX_SIZE);
//         tf.scale.x = size;
//         tf.scale.y = size;
//         // Update color
//         let color = match state {
//             SlimeState::Normal => Color::RED,
//             SlimeState::Eating => Color::PINK,
//         };
//         if let DrawMode::Fill(fill) = draw_mode.as_mut() {
//             fill.color = color;
//         }
//     }
// }

#[derive(Component)]
struct HoverDisplay;

/// Map from `Slime` entity => `HoverDisplay` entity
struct HoveredSlimes(HashMap<Entity, Entity>);

fn hover_actions(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_position: Res<MousePosition>,
    mut hovered_slimes: ResMut<HoveredSlimes>,
    mut hover_query: Query<(Entity, &mut Transform), (With<HoverDisplay>, Without<Slime>)>,
    slime_query: Query<(Entity, &Transform), With<Slime>>,
) {
    if let Some(mouse_pos) = mouse_position.0 {
        let window = windows.get_primary().unwrap();
        let window_2 = Vec2::new(window.width() / 2.0, window.height() / 2.0);
        let mouse_pos = mouse_pos - window_2;
        let mut current_hovered_slimes = HashSet::new();

        for (slime_entity, slime_tf) in slime_query.iter() {
            let slime_pos = slime_tf.translation.xy();
            let hover_range = slime_tf.scale.x + SLIME_VISION_RANGE;
            let target_hover = hovered_slimes.0.get(&slime_entity).copied();
            if mouse_pos.distance(slime_pos) <= hover_range {
                current_hovered_slimes.insert(slime_entity);
                if let Some(target_hover) = target_hover {
                    // Modify HoverDisplay position (follow the slime)
                    if let Ok((_hovered_entity, mut hovered_tf)) = hover_query.get_mut(target_hover)
                    {
                        hovered_tf.translation.x = slime_pos.x;
                        hovered_tf.translation.y = slime_pos.y;
                        hovered_tf.scale = Vec3::new(hover_range, hover_range, 1.0);
                    } else {
                        warn!(
                            "Tried to retrieve an invalid hovered entity: {:?} {:?}",
                            target_hover, hovered_slimes.0
                        );
                    }
                } else {
                    // Create a new HoverDisplay
                    let shape_bundle = SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 0.08, 0.58, 0.5),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(slime_pos.x, slime_pos.y, 0.0)
                            .with_scale(Vec3::new(hover_range, hover_range, 1.0)),
                        ..Default::default()
                    };
                    let id = commands
                        .spawn_bundle(shape_bundle)
                        .insert(HoverDisplay)
                        .id();
                    hovered_slimes.0.insert(slime_entity, id);
                }
            }
        }
        // Remove non used HoverDiplays
        let remove_slimes = hovered_slimes
            .0
            .keys()
            .filter(|k| !current_hovered_slimes.contains(k))
            .copied()
            .collect::<Vec<_>>();
        for remove_slime in remove_slimes {
            if let Some(target_hover) = hovered_slimes.0.remove(&remove_slime) {
                commands.entity(target_hover).despawn();
            }
        }
    } else {
        // Remove all HoverDisplay
        hovered_slimes.0.clear();
        for (hovered_entity, _) in hover_query.iter_mut() {
            commands.entity(hovered_entity).despawn();
        }
    }
}
