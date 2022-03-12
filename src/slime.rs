//! # Slime entity.
#![doc = include_str!("../docs/slime.md")]
use crate::utils::random_screen_position;
use macroquad::{prelude::*, rand::gen_range};

/// When slime is below this threshold, its free to move without energy cost.
const FREE_MOVEMENT_TH: f32 = 5.0;
/// How often (seconds) slimes consume 1 energy.
const TIME_COST: f64 = 0.5;
/// Energy required to jump.
const JUMP_COST: f32 = 5.0;
/// Every time a slime collects this amount of energy, it can evolve.
const EvOLVE_REQUIREMENT: f32 = 50.0;
/// Slimes need at least this amount of energy to be able to breed.
const BREEDING_REQUIREMENT: f32 = 100.0;
/// Adter this amount of seconds without eating, a slime will die.
const STARVATION_TIME: f64 = 5.0;

pub struct Slime {
    pub position: Vec2,
    speed_factor: f32,
    energy: f32,
    step_cost: f32,
}

impl Slime {
    pub fn spawn(speed_factor: f32, energy: f32, step_cost: f32) -> Self {
        Self {
            position: random_screen_position(),
            speed_factor,
            energy,
            step_cost,
        }
    }

    /// Get size as proportional to its energy.
    pub fn size(&self) -> f32 {
        (self.energy / 50.0).clamp(2.5, 10.0)
    }
}

pub struct SlimeController {
    speed_factor: f32,
    initial_energy: f32,
    initial_step_cost: f32,
    pub population: Vec<Slime>,
}

impl SlimeController {
    pub fn new(speed_factor: f32, initial_energy: f32, initial_step_cost: f32) -> Self {
        Self {
            speed_factor,
            initial_energy,
            initial_step_cost,
            population: Vec::new(),
        }
    }

    pub fn spawn_one(&mut self) {
        self.population.push(Slime::spawn(
            self.speed_factor,
            self.initial_energy,
            self.initial_step_cost,
        ));
    }

    pub fn spawn_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.spawn_one())
    }
}
