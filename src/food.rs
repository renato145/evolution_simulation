//! # Food entity.
#![doc = include_str!("../docs/food.md")]
use crate::utils::{random_screen_position, wrap_around};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::f32::consts::PI;

pub const FOOD_SIZE: f32 = 3.0;

pub struct Food {
    pub position: Vec2,
    pub energy: f32,
    _speed_factor: f32,
    speed: Vec2,
}

impl Food {
    pub fn spawn(energy_range: (f32, f32), speed_range: (f32, f32)) -> Self {
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
            _speed_factor: speed_factor,
            speed,
        }
    }
}

pub struct FoodController {
    /// Spawn time
    pub spawn_time: f32,
    /// Maximum number of food instances that can exist at the same time.
    pub limit: f32,
    pub energy_range: (f32, f32),
    pub speed_range: (f32, f32),
    time: f32,
    pub last_spawn_time: f32,
    pub population: Vec<Food>,
}

impl FoodController {
    pub fn new(
        spawn_time: f32,
        limit: f32,
        energy_range: (f32, f32),
        speed_range: (f32, f32),
    ) -> Self {
        Self {
            spawn_time,
            limit,
            energy_range,
            speed_range,
            time: 0.0,
            last_spawn_time: 0.0,
            population: Vec::with_capacity(limit as usize),
        }
    }

    /// Get FoodController's limit as usize
    pub fn limit(&self) -> usize {
        self.limit.ceil() as usize
    }

    pub fn set_time(&mut self, time: f32) {
        self.time = time;
    }

    pub fn spawn_one(&mut self) {
        self.population
            .push(Food::spawn(self.energy_range, self.speed_range));
    }

    pub fn spawn_n(&mut self, n: usize) {
        let n = self.limit().saturating_sub(self.population.len()).min(n);
        (0..n).for_each(|_| self.spawn_one())
    }

    /// Check timer to spawn one food instance.
    pub fn check_spawn(&mut self) {
        if (self.time - self.last_spawn_time) >= self.spawn_time {
            if self.limit() > self.population.len() {
                self.spawn_one();
            }
            self.last_spawn_time = self.time;
        }
    }

    /// 1. Update all food positions.
    /// 2. Check to spawn more food.
    pub fn update_step(&mut self) {
        for food in self.population.iter_mut() {
            food.position += food.speed;
            food.position = wrap_around(&food.position);
        }
        self.check_spawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Food {
        pub fn create_test(position: Vec2) -> Self {
            Self {
                position,
                energy: 1.0,
                _speed_factor: 1.0,
                speed: vec2(0.0, 0.0),
            }
        }
    }
}
