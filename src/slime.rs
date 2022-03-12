//! # Slime entity.
#![doc = include_str!("../docs/slime.md")]
use std::ops::Div;

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
const JUMP_COST: f32 = 2.5;
/// Jump distance.
const JUMP_DISTANCE: f32 = 10.0;
/// Minimum energy required to be able to jump.
const JUMP_REQUIREMENT: f32 = 25.0;
/// Every time a slime collects this amount of energy, it can evolve.
const EVOLVE_REQUIREMENT: f32 = 50.0;
/// Slimes need at least this amount of energy to be able to breed.
const BREEDING_REQUIREMENT: f32 = 50.0;
/// Time cooldown for slimes to breed.
const BREEDING_COOLDOWN: f64 = 5.0;

#[derive(Clone, PartialEq, Eq)]
pub enum SlimeState {
    Normal,
    Jumping,
    Breeding,
}

#[derive(Clone)]
pub struct Slime {
    pub position: Vec2,
    pub state: SlimeState,
    speed_factor: f32,
    energy: f32,
    size: f32,
    step_cost: f32,
    vision_range: f32,
    jump_cooldown: f64,
    last_jump: f64,
    last_breed: f64,
}

impl Slime {
    pub fn spawn(
        speed_factor: f32,
        energy: f32,
        step_cost: f32,
        vision_range: f32,
        jump_cooldown: f64,
    ) -> Self {
        let t = get_time();
        let mut slime = Self {
            position: random_screen_position(),
            state: SlimeState::Normal,
            speed_factor,
            energy,
            size: 0.0,
            step_cost,
            vision_range,
            jump_cooldown,
            last_jump: t,
            last_breed: t,
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

    /// Checks the nearst position and returns its index and distance.
    fn nearest_position(&self, positions: impl Iterator<Item = Vec2>) -> Option<(usize, f32)> {
        positions
            .enumerate()
            .map(|(i, pos)| (i, self.position.distance(pos)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Checks the nearest food and returns its index and distance.
    fn nearest_food(&self, foods: &[Food]) -> Option<(usize, f32)> {
        self.nearest_position(foods.iter().map(|f| f.position))
    }

    /// Checks the nearest other slime able to breed and returns its index and distance.
    /// * `idx` - Index of the current slime in `slimes`.
    fn nearest_breeding_slime(&self, idx: usize, slimes: &[Slime]) -> Option<(usize, f32)> {
        let (idxs, positions): (Vec<_>, Vec<_>) = slimes
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if (i != idx) && (s.is_breed_ready()) {
                    Some((i, s.position))
                } else {
                    None
                }
            })
            .unzip();

        if let Some((i, distance)) = self.nearest_position(positions.into_iter()) {
            Some((idxs[i], distance))
        } else {
            None
        }
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
            let mult = (self.energy / 100.0).max(1.0);
            self.add_energy(-self.step_cost * mult);
        }
    }

    fn is_jump_ready(&self) -> bool {
        (self.energy >= JUMP_REQUIREMENT) && ((get_time() - self.last_jump) >= self.jump_cooldown)
    }

    fn is_breed_ready(&self) -> bool {
        (self.state != SlimeState::Breeding)
            && (self.energy >= BREEDING_REQUIREMENT)
            && ((get_time() - self.last_breed) >= BREEDING_COOLDOWN)
    }

    /// Returns a new `Slime` with an initial energy.
    fn breed(&mut self, partner: &mut Self, energy: f32) -> Self {
        self.last_breed = get_time();
        self.state = SlimeState::Breeding;
        self.add_energy(-energy);
        partner.last_breed = get_time();
        partner.state = SlimeState::Breeding;
        partner.add_energy(-energy);
        let t = get_time();
        let mut child = Self {
            position: self.position,
            state: SlimeState::Normal,
            speed_factor: self.speed_factor,
            energy,
            size: 0.0,
            step_cost: self.step_cost,
            vision_range: self.vision_range,
            jump_cooldown: self.jump_cooldown,
            last_jump: t,
            last_breed: t,
        };
        child.update_size();
        child
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
    /// 1. Update slime position to get close its nearest food in vision range or nearest other slime if ready to breed.
    /// 2. If on top a food, eat it.
    /// 3. If possible try to breed.
    /// 4. If didn't eat or breed, check if slime can jump.
    /// At the end of the loop childs (step 3) are added to population.
    pub fn update_step(&mut self, foods: &mut Vec<Food>) {
        self.check_time_cost();
        self.reset_slime_states();
        let n = self.population.len();
        let mut childs = Vec::new();
        for idx in 0..n {
            // Step 1: Move
            let mut slime = self.population[idx].clone();
            let mut target_position_distance = None;
            let breed_ready = slime.is_breed_ready();
            let mut breeding_target = None;

            // - Get target position distance
            if breed_ready {
                if let Some((i, distance)) = slime.nearest_breeding_slime(idx, &self.population) {
                    if (distance - slime.size) <= slime.vision_range {
                        target_position_distance = Some((self.population[i].position, distance));
                        breeding_target = Some(i);
                    }
                }
            }
            if target_position_distance.is_none() {
                if let Some((i, distance)) = slime.nearest_food(foods) {
                    if (distance - slime.size) <= slime.vision_range {
                        target_position_distance = Some((foods[i].position, distance));
                    }
                }
            }

            // - Move to target
            if let Some((position, distance)) = target_position_distance {
                let direction = get_angle_direction(slime.position, position);
                let speed = polar_to_cartesian(slime.speed_factor.min(distance), direction);
                slime.position += speed;
                slime.position = wrap_around(&slime.position);
                slime.apply_movement_cost();
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

            // Step 3: Breed
            if breed_ready {
                if let Some(i) = breeding_target {
                    let partner = &mut self.population[i];
                    if slime.is_point_inside(partner.position, 0.0) {
                        childs.push(slime.breed(partner, self.initial_energy));
                    }
                }
            }

            // Step 4: Jump
            if !did_eat && (slime.state != SlimeState::Breeding) && slime.is_jump_ready() {
                if let Some((i, distance)) = slime.nearest_food(foods) {
                    if (distance - slime.size) <= JUMP_DISTANCE {
                        let nearest_food = &foods[i];
                        slime.position = nearest_food.position;
                        slime.add_energy(nearest_food.energy - JUMP_COST);
                        foods.remove(i);
                        slime.last_jump = get_time();
                        slime.state = SlimeState::Jumping;
                    }
                }
            }

            self.population[idx] = slime;
        }

        // Add childs to population
        self.population.append(&mut childs);
    }

    pub fn reset_time(&mut self) {
        self.last_time_cost = get_time();
    }

    fn reset_slime_states(&mut self) {
        self.population
            .iter_mut()
            .for_each(|s| s.state = SlimeState::Normal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Slime {
        pub fn create_test(position: Vec2) -> Self {
            Self {
                position,
                state: SlimeState::Normal,
                speed_factor: 1.0,
                energy: 10.0,
                size: 1.0,
                step_cost: 0.1,
                vision_range: 10.0,
                jump_cooldown: 2.0,
                last_jump: 0.0,
                last_breed: 0.0,
            }
        }
    }

    #[test]
    fn nearest_position() {
        let slime = Slime::create_test(vec2(5.0, 5.0));
        let positions = [vec2(0.0, 0.0), vec2(2.0, 2.0), vec2(10.0, 10.0)];
        let (i, distance) = slime.nearest_position(positions.into_iter()).unwrap();
        println!("distance={}", distance);
        assert_eq!(i, 1);
    }
}
