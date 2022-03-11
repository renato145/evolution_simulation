//! # Food entity.
#![doc = include_str!("../docs/food.md")]
use crate::utils::random_screen_position;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::f32::consts::PI;

pub struct Food {
    pub position: Vec2,
    energy: usize,
    speed_factor: f32,
    speed: Vec2,
}

impl Food {
    pub fn spawn(energy_range: (usize, usize), speed_range: (f32, f32)) -> Self {
        let energy = gen_range(energy_range.0, energy_range.1);
        // Get speed as proportional to energy
        let speed_factor = speed_range.0
            + ((energy - energy_range.0) as f32 / (energy_range.1 - energy_range.0) as f32
                * (speed_range.1 - speed_range.0));
        // Get random direction angle
        let direction = gen_range(0.0, PI * 2.0);
        let speed = polar_to_cartesian(speed_factor, direction);
        Self {
            position: random_screen_position(),
            energy,
            speed_factor,
            speed,
        }
    }
}

pub struct FoodController {
    /// Spawn time in seconds
    spawn_time: f64,
    /// Maximum number of food instances that can exist at the same time.
    limit: usize,
    energy_range: (usize, usize),
    speed_range: (f32, f32),
    last_spawn_time: f64,
    pub population: Vec<Food>,
}

impl FoodController {
    pub fn new(
        spawn_time: f64,
        limit: usize,
        energy_range: (usize, usize),
        speed_range: (f32, f32),
    ) -> Self {
        Self {
            spawn_time,
            limit,
            energy_range,
            speed_range,
            last_spawn_time: get_time(),
            population: Vec::with_capacity(limit),
        }
    }

    pub fn reset_time(&mut self) {
        self.last_spawn_time = get_time();
    }

    pub fn spawn(&mut self) {
        self.population
            .push(Food::spawn(self.energy_range, self.speed_range));
    }

    pub fn spawn_n(&mut self, n: usize) {
        let n = self.limit.saturating_sub(self.population.len()).min(n);
        (0..n).for_each(|_| self.spawn())
    }

    pub fn update_food_positions(&mut self) {
        for food in self.population.iter_mut() {
            food.position += food.speed;
        }
    }
}
