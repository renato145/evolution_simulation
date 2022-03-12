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
const TIME_COST: f64 = 0.5;
/// Energy required to jump.
const JUMP_COST: f32 = 5.0;
/// Every time a slime collects this amount of energy, it can evolve.
const EVOLVE_REQUIREMENT: f32 = 50.0;
/// Slimes need at least this amount of energy to be able to breed.
const BREEDING_REQUIREMENT: f32 = 100.0;
/// Adter this amount of seconds without eating, a slime will die.
const STARVATION_TIME: f64 = 5.0;

pub struct Slime {
    pub position: Vec2,
    speed_factor: f32,
    energy: f32,
    size: f32,
    step_cost: f32,
    vision_range: f32,
}

impl Slime {
    pub fn spawn(speed_factor: f32, energy: f32, step_cost: f32, vision_range: f32) -> Self {
        let mut slime = Self {
            position: random_screen_position(),
            speed_factor,
            energy,
            size: 0.0,
            step_cost,
            vision_range,
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
        self.size = (self.energy / 50.0).clamp(2.5, 10.0);
    }

    /// Returns nearest food and distance
    fn nearest_food<'a>(&self, foods: &'a [Food]) -> Option<(&'a Food, f32)> {
        foods
            .iter()
            .map(|f| (f, self.position.distance(f.position)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Returns if point is inside the Slime
    pub fn is_point_inside(&self, point: Vec2, padding: f32) -> bool {
        self.position.distance(point) <= (self.size + padding)
    }

    fn add_energy(&mut self, energy: f32) {
        self.energy += energy;
        self.update_size();
    }

    /// Get the slime's energy.
    pub fn energy(&self) -> f32 {
        self.energy
    }
}

pub struct SlimeController {
    speed_factor: f32,
    initial_energy: f32,
    initial_step_cost: f32,
    initial_vision_range: f32,
    pub population: Vec<Slime>,
}

impl SlimeController {
    pub fn new(
        speed_factor: f32,
        initial_energy: f32,
        initial_step_cost: f32,
        initial_vision_range: f32,
    ) -> Self {
        Self {
            speed_factor,
            initial_energy,
            initial_step_cost,
            initial_vision_range,
            population: Vec::new(),
        }
    }

    pub fn spawn_one(&mut self) {
        self.population.push(Slime::spawn(
            self.speed_factor,
            self.initial_energy,
            self.initial_step_cost,
            self.initial_vision_range,
        ));
    }

    pub fn spawn_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.spawn_one())
    }

    /// For each slime:
    /// 1. Update slime position to get close its nearest food in vision range.
    /// 2. If on top a food, eat it.
    pub fn update_step(&mut self, foods: &mut Vec<Food>) {
        for slime in self.population.iter_mut() {
            // Step 1: Move
            if let Some((nearest_food, distance)) = slime.nearest_food(foods) {
                if (distance - slime.size) <= slime.vision_range {
                    let direction = get_angle_direction(slime.position, nearest_food.position);
                    let speed = polar_to_cartesian(slime.speed_factor.min(distance), direction);
                    slime.position += speed;
                    slime.position = wrap_around(&slime.position);
                }
            }
            // Step 2: Eat
            let mut i = 0;
            while i < foods.len() {
                if slime.is_point_inside(foods[i].position, 0.0) {
                    slime.add_energy(foods[i].energy);
                    foods.remove(i);
                } else {
                    i += 1;
                }
            }
        }
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
        let (nearest_food, distance) = slime.nearest_food(&foods).unwrap();
        println!("distance={}", distance);
        assert_eq!(nearest_food.position, positions[1]);
    }
}
