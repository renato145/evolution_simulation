//! # Slime entity.
#![doc = include_str!("../docs/slime.md")]
use crate::{
    food::Food,
    utils::{get_angle_direction, random_screen_position, wrap_around},
};
use macroquad::prelude::*;

/// When slime is below this threshold, its free to move without energy cost.
const FREE_MOVEMENT_TH: f32 = 5.0;
/// How often (seconds) slimes consume 1 energy.
const TIME_COST_FREQ: f64 = 1.0;
/// Energy cost to jump.
const JUMP_COST: f32 = 5.0;
/// Jump distance.
const JUMP_DISTANCE: f32 = 10.0;
/// Minimum energy required to be able to jump.
const JUMP_REQUIREMENT: f32 = 25.0;
/// Every time a slime collects this amount of energy, it can evolve.
const EVOLVE_REQUIREMENT: f32 = 50.0;
/// Slimes need at least this amount of energy to be able to breed.
const BREEDING_REQUIREMENT: f32 = 100.0;

pub struct Slime {
    pub position: Vec2,
    speed_factor: f32,
    energy: f32,
    size: f32,
    step_cost: f32,
    vision_range: f32,
    jump_cooldown: f64,
    last_jump: f64,
    is_jumping: bool,
}

impl Slime {
    pub fn spawn(
        speed_factor: f32,
        energy: f32,
        step_cost: f32,
        vision_range: f32,
        jump_cooldown: f64,
    ) -> Self {
        let mut slime = Self {
            position: random_screen_position(),
            speed_factor,
            energy,
            size: 0.0,
            step_cost,
            vision_range,
            jump_cooldown,
            is_jumping: false,
            last_jump: get_time(),
        };
        slime.update_size();
        slime
    }

    /// Get the slime's step cost.
    pub fn step_cost(&self) -> f32 {
        self.step_cost
    }

    /// Get the slime's size.
    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn size_vision(&self) -> f32 {
        self.size + self.vision_range
    }

    /// Set size as proportional to its energy.
    pub fn update_size(&mut self) {
        self.size = (self.energy / 50.0).clamp(2.5, 30.0);
    }

    /// Checks the nearst food and returns its index, reference and distance.
    fn nearest_food<'a>(&self, foods: &'a [Food]) -> Option<(usize, &'a Food, f32)> {
        foods
            .iter()
            .enumerate()
            .map(|(i, f)| (i, f, self.position.distance(f.position)))
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
    }

    /// Returns if point is inside the Slime
    pub fn is_point_inside(&self, point: Vec2, padding: f32) -> bool {
        self.position.distance(point) <= (self.size + padding)
    }

    /// Get the slime's energy.
    pub fn energy(&self) -> f32 {
        self.energy
    }

    fn add_energy(&mut self, energy: f32) {
        self.energy += energy;
        self.update_size();
    }

    fn apply_movement_cost(&mut self) {
        if self.energy > FREE_MOVEMENT_TH {
            self.add_energy(-self.step_cost);
        }
    }

    fn is_jump_ready(&self) -> bool {
        (get_time() - self.last_jump) >= self.jump_cooldown
    }

    /// Get the slime's is jumping.
    pub fn is_jumping(&self) -> bool {
        self.is_jumping
    }
}

pub struct SlimeController {
    speed_factor: f32,
    initial_energy: f32,
    initial_step_cost: f32,
    initial_vision_range: f32,
    initial_jump_cooldown: f64,
    last_time_cost: f64,
    pub population: Vec<Slime>,
}

impl SlimeController {
    pub fn new(
        speed_factor: f32,
        initial_energy: f32,
        initial_step_cost: f32,
        initial_vision_range: f32,
        initial_jump_cooldown: f64,
    ) -> Self {
        Self {
            speed_factor,
            initial_energy,
            initial_step_cost,
            initial_vision_range,
            initial_jump_cooldown,
            last_time_cost: get_time(),
            population: Vec::new(),
        }
    }

    pub fn spawn_one(&mut self) {
        self.population.push(Slime::spawn(
            self.speed_factor,
            self.initial_energy,
            self.initial_step_cost,
            self.initial_vision_range,
            self.initial_jump_cooldown,
        ));
    }

    pub fn spawn_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.spawn_one())
    }

    /// Check timer for time cost.
    pub fn check_time_cost(&mut self) {
        let t = get_time();
        if (t - self.last_time_cost) >= TIME_COST_FREQ {
            let mut i = 0;
            while i < self.population.len() {
                self.population[i].add_energy(-1.0);
                if self.population[i].energy <= 0.0 {
                    self.population.remove(i);
                } else {
                    i += 1;
                }
            }
            self.last_time_cost = t;
        }
    }

    /// Check time cost, then, for each slime:
    /// 1. Update slime position to get close its nearest food in vision range.
    /// 2. If on top a food, eat it.
    /// 3. If didn't eat check if slime can jump.
    pub fn update_step(&mut self, foods: &mut Vec<Food>) {
        self.check_time_cost();
        for slime in self.population.iter_mut() {
            // Step 1: Move
            if let Some((_, nearest_food, distance)) = slime.nearest_food(foods) {
                if (distance - slime.size) <= slime.vision_range {
                    let direction = get_angle_direction(slime.position, nearest_food.position);
                    let speed = polar_to_cartesian(slime.speed_factor.min(distance), direction);
                    slime.position += speed;
                    slime.position = wrap_around(&slime.position);
                    slime.apply_movement_cost();
                }
            }
            // Step 2: Eat
            let mut i = 0;
            let mut did_eat = false;
            while i < foods.len() {
                if slime.is_point_inside(foods[i].position, 0.0) {
                    slime.add_energy(foods[i].energy);
                    foods.remove(i);
                    did_eat = true;
                } else {
                    i += 1;
                }
            }
            // Step 3: Jump
            if !did_eat && slime.is_jump_ready() {
                if let Some((i, nearest_food, distance)) = slime.nearest_food(foods) {
                    if (distance - slime.size) <= JUMP_DISTANCE {
                        slime.position = nearest_food.position;
                        slime.add_energy(nearest_food.energy);
                        foods.remove(i);
                        slime.last_jump = get_time();
                        slime.is_jumping = true;
                    }
                }
            } else {
                slime.is_jumping = false;
            }
        }
    }

    pub fn reset_time(&mut self) {
        self.last_time_cost = get_time();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Slime {
        pub fn create_test(position: Vec2) -> Self {
            Self {
                position,
                speed_factor: 1.0,
                energy: 10.0,
                size: 1.0,
                step_cost: 0.1,
                vision_range: 10.0,
                jump_cooldown: 2.0,
                last_jump: 0.0,
                is_jumping: false,
            }
        }
    }

    #[test]
    fn nearest_food_works() {
        let slime = Slime::create_test(vec2(5.0, 5.0));
        let positions = [vec2(0.0, 0.0), vec2(2.0, 2.0), vec2(10.0, 10.0)];
        let foods = positions
            .clone()
            .into_iter()
            .map(|pos| Food::create_test(pos))
            .collect::<Vec<_>>();
        let (_i, nearest_food, distance) = slime.nearest_food(&foods).unwrap();
        println!("distance={}", distance);
        assert_eq!(nearest_food.position, positions[1]);
    }
}
