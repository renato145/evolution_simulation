//! # Slime entity.
#![doc = include_str!("../docs/slime.md")]
use crate::{
    food::Food,
    utils::{get_angle_direction, random_screen_position, wrap_around},
};
use macroquad::{prelude::*, rand::gen_range};

/// When slime is below this threshold, its free to move without energy cost.
const FREE_MOVEMENT_TH: f32 = 10.0;
/// How often (time steps) slimes consume 1 energy.
const TIME_COST_FREQ: f32 = 10.0;
/// Energy cost to jump.
const JUMP_COST: f32 = 3.0;
/// Jump distance.
const JUMP_DISTANCE: f32 = 10.0;
/// Minimum energy required to be able to jump.
const JUMP_REQUIREMENT: f32 = 25.0;
/// Every time a slime collects this amount of energy, it can evolve.
const EVOLVE_REQUIREMENT: f32 = 40.0;
/// Maximum number of skills.
const EVOLVE_LIMIT: usize = 21;
/// Slimes need at least this amount of energy to be able to breed.
const BREEDING_REQUIREMENT: f32 = 120.0;
/// Time cooldown for slimes to breed.
const BREEDING_COOLDOWN: f32 = 1000.0;

#[derive(Clone, PartialEq, Eq)]
pub enum SlimeState {
    Normal,
    Jumping,
    Breeding,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SkillType {
    /// Increase the range of vision to detect food and increases a bit the speed.
    Vision,
    /// Reduces the energy needed to move around.
    Efficiency,
    /// Reduces jump cooldown.
    Jumper,
}

impl SkillType {
    fn random() -> Self {
        match gen_range(0, 3) {
            0 => Self::Vision,
            1 => Self::Efficiency,
            2 => Self::Jumper,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Skills {
    pub vision: usize,
    pub efficiency: usize,
    pub jumper: usize,
}

impl From<(usize, usize, usize)> for Skills {
    fn from(vej: (usize, usize, usize)) -> Self {
        Self {
            vision: vej.0,
            efficiency: vej.1,
            jumper: vej.2,
        }
    }
}

impl Skills {
    fn new() -> Self {
        Self {
            vision: 0,
            efficiency: 0,
            jumper: 0,
        }
    }

    fn count_levels(&self) -> usize {
        self.vision + self.efficiency + self.jumper
    }

    fn unique_skills(&self) -> usize {
        let zero_count = (self.vision == 0) as usize
            + (self.efficiency == 0) as usize
            + (self.jumper == 0) as usize;
        3 - zero_count
    }

    fn add_skill(&mut self, skill_type: SkillType) {
        match skill_type {
            SkillType::Vision => self.vision += 1,
            SkillType::Efficiency => self.efficiency += 1,
            SkillType::Jumper => self.jumper += 1,
        }
    }

    /// Chooses a random skill and returns it with the level reduced by 1/3 (rounded up).
    fn inherit(&self) -> Option<Skills> {
        match self.unique_skills() {
            0 => None,
            1 => {
                let mut vej = (0, 0, 0);
                if self.vision > 0 {
                    vej.0 += (self.vision as f32 / 3.0).ceil() as usize
                } else if self.efficiency > 0 {
                    vej.1 += (self.efficiency as f32 / 3.0).ceil() as usize
                } else {
                    vej.2 += (self.jumper as f32 / 3.0).ceil() as usize
                }
                Some(vej.into())
            }
            _ => {
                let mut vej = (0, 0, 0);
                let i = gen_range(0, self.count_levels());
                if i <= self.vision {
                    vej.0 += (self.vision as f32 / 3.0).ceil() as usize
                } else if i <= (self.vision + self.efficiency) {
                    vej.1 += (self.efficiency as f32 / 3.0).ceil() as usize
                } else {
                    vej.2 += (self.jumper as f32 / 3.0).ceil() as usize
                }
                Some(vej.into())
            }
        }
    }

    fn merge(mut self, rhs: Self) -> Self {
        self.vision += rhs.vision;
        self.efficiency += rhs.efficiency;
        self.jumper += rhs.jumper;
        self
    }
}

#[derive(Clone)]
pub struct Slime {
    pub position: Vec2,
    pub state: SlimeState,
    pub skills: Skills,
    speed_factor: f32,
    energy: f32,
    size: f32,
    step_cost: f32,
    vision_range: f32,
    jump_cooldown: f32,
    last_jump: f32,
    last_breed: f32,
    next_skill_goal: f32,
    skill_path: SkillType,
}

impl Slime {
    pub fn new(
        position: Vec2,
        speed_factor: f32,
        energy: f32,
        step_cost: f32,
        vision_range: f32,
        jump_cooldown: f32,
    ) -> Self {
        let mut slime = Self {
            position,
            state: SlimeState::Normal,
            skills: Skills::new(),
            speed_factor,
            energy,
            size: 0.0,
            step_cost,
            vision_range,
            jump_cooldown,
            last_jump: 0.0,
            last_breed: 0.0,
            next_skill_goal: EVOLVE_REQUIREMENT,
            skill_path: SkillType::random(),
        };
        slime.update_size();
        slime
    }

    pub fn spawn(
        speed_factor: f32,
        energy: f32,
        step_cost: f32,
        vision_range: f32,
        jump_cooldown: f32,
    ) -> Self {
        Self::new(
            random_screen_position(),
            speed_factor,
            energy,
            step_cost,
            vision_range,
            jump_cooldown,
        )
    }

    /// Get the slime's size.
    pub fn size(&self) -> f32 {
        self.size
    }

    /// Get the slime's speed factor considering skill modifications.
    /// Max skill augmentation will increment it to 2x.
    pub fn speed_factor(&self) -> f32 {
        self.speed_factor * (1.0 + (self.skills.vision as f32) / (EVOLVE_LIMIT as f32) * 1.0)
    }

    /// Get the slime's vision range considering skill modifications.
    /// Max skill augmentation will increment it to 5x.
    pub fn vision_range(&self) -> f32 {
        self.vision_range * (1.0 + (self.skills.vision as f32) / (EVOLVE_LIMIT as f32) * 4.0)
    }

    pub fn size_vision(&self) -> f32 {
        self.size + self.vision_range()
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
    fn nearest_breeding_slime(
        &self,
        idx: usize,
        slimes: &[Slime],
        time: f32,
    ) -> Option<(usize, f32)> {
        let (idxs, positions): (Vec<_>, Vec<_>) = slimes
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if (i != idx) && (s.is_breed_ready(time)) {
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

    /// Get the slime's step cost considering skill modifications.
    /// Max skill augmentation will decrease it by 1/3.
    pub fn step_cost(&self) -> f32 {
        self.step_cost / (1.0 + (self.skills.efficiency as f32) / (EVOLVE_LIMIT as f32) * 2.0)
    }

    fn apply_movement_cost(&mut self) {
        if self.energy > FREE_MOVEMENT_TH {
            let mult = (self.energy / 100.0).max(1.0);
            self.add_energy(-self.step_cost() * mult);
        }
    }

    /// Get the slime's jump cooldown considering skill modifications.
    /// Max skill augmentation will decrease it by 1/5.
    pub fn jump_cooldown(&self) -> f32 {
        self.jump_cooldown / (1.0 + (self.skills.jumper as f32) / (EVOLVE_LIMIT as f32) * 4.0)
    }

    fn is_jump_ready(&self, time: f32) -> bool {
        (self.energy >= JUMP_REQUIREMENT) && ((time - self.last_jump) >= self.jump_cooldown())
    }

    fn is_breed_ready(&self, time: f32) -> bool {
        (self.state != SlimeState::Breeding)
            && (self.energy >= BREEDING_REQUIREMENT)
            && ((time - self.last_breed) >= BREEDING_COOLDOWN)
    }

    fn is_evolve_ready(&self) -> bool {
        self.energy >= self.next_skill_goal
    }

    /// Returns a new `Slime` with an initial energy. It will randomly inherit one skill
    /// from each parent at random reducing its level by 1/3 (rounding up).
    fn breed(&mut self, partner: &mut Self, energy: f32, time: f32) -> Self {
        self.last_breed = time;
        self.state = SlimeState::Breeding;
        self.add_energy(-energy);
        partner.last_breed = time;
        partner.state = SlimeState::Breeding;
        partner.add_energy(-energy);
        let mut child = Self::new(
            self.position,
            self.speed_factor,
            energy,
            self.step_cost,
            self.vision_range,
            self.jump_cooldown,
        );
        let skills = match (self.skills.inherit(), partner.skills.inherit()) {
            (None, None) => Skills::new(),
            (None, Some(s)) => s,
            (Some(s), None) => s,
            (Some(sa), Some(sb)) => sa.merge(sb),
        };
        child.skills = skills;
        child.next_skill_goal = if child.skills.count_levels() == EVOLVE_LIMIT {
            std::f32::MAX
        } else {
            (child.skills.count_levels() + 1) as f32 * EVOLVE_REQUIREMENT
        };
        child
    }
}

pub struct SlimeController {
    speed_factor: f32,
    initial_energy: f32,
    initial_step_cost: f32,
    initial_vision_range: f32,
    initial_jump_cooldown: f32,
    time: f32,
    last_time_cost: f32,
    pub population: Vec<Slime>,
}

impl SlimeController {
    pub fn new(
        speed_factor: f32,
        initial_energy: f32,
        initial_step_cost: f32,
        initial_vision_range: f32,
        initial_jump_cooldown: f32,
    ) -> Self {
        Self {
            speed_factor,
            initial_energy,
            initial_step_cost,
            initial_vision_range,
            initial_jump_cooldown,
            time: 0.0,
            last_time_cost: 0.0,
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
        if (self.time - self.last_time_cost) >= TIME_COST_FREQ {
            let mut i = 0;
            while i < self.population.len() {
                self.population[i].add_energy(-1.0);
                if self.population[i].energy <= 0.0 {
                    self.population.remove(i);
                } else {
                    i += 1;
                }
            }
            self.last_time_cost = self.time;
        }
    }

    /// Check time cost, then, for each slime:
    /// 1. Update slime position to get close its nearest food in vision range or nearest other slime if ready to breed.
    /// 2. If on top a food, eat it.
    /// 3. If possible try to breed.
    /// 4. If didn't eat or breed, check if slime can jump.
    /// 5. Check if it can evolve.
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
            let breed_ready = slime.is_breed_ready(self.time);
            let mut breeding_target = None;

            // - Get target position distance
            if breed_ready {
                if let Some((i, distance)) =
                    slime.nearest_breeding_slime(idx, &self.population, self.time)
                {
                    if (distance - slime.size) <= slime.vision_range() {
                        target_position_distance = Some((self.population[i].position, distance));
                        breeding_target = Some(i);
                    }
                }
            }
            if target_position_distance.is_none() {
                if let Some((i, distance)) = slime.nearest_food(foods) {
                    if (distance - slime.size) <= slime.vision_range() {
                        target_position_distance = Some((foods[i].position, distance));
                    }
                }
            }

            // - Move to target
            if let Some((position, distance)) = target_position_distance {
                let direction = get_angle_direction(slime.position, position);
                let speed = polar_to_cartesian(slime.speed_factor().min(distance), direction);
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
                        childs.push(slime.breed(partner, self.initial_energy, self.time));
                    }
                }
            }

            // Step 4: Jump
            if !did_eat && (slime.state != SlimeState::Breeding) && slime.is_jump_ready(self.time) {
                if let Some((i, distance)) = slime.nearest_food(foods) {
                    if (distance - slime.size) <= JUMP_DISTANCE {
                        let nearest_food = &foods[i];
                        slime.position = nearest_food.position;
                        slime.add_energy(nearest_food.energy - JUMP_COST);
                        foods.remove(i);
                        slime.last_jump = self.time;
                        slime.state = SlimeState::Jumping;
                    }
                }
            }

            // Step 5: Evolve
            if slime.is_evolve_ready() {
                slime.skills.add_skill(slime.skill_path);
                if slime.skills.count_levels() >= EVOLVE_LIMIT {
                    slime.next_skill_goal = std::f32::MAX;
                } else {
                    slime.next_skill_goal += EVOLVE_REQUIREMENT;
                }
            }

            self.population[idx] = slime;
        }

        // Add childs to population
        self.population.append(&mut childs);
    }

    pub fn set_time(&mut self, time: f32) {
        self.time = time;
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
            Self::new(position, 1.0, 10.0, 0.1, 10.0, 2.0)
        }
    }

    #[test]
    fn nearest_position_works() {
        let slime = Slime::create_test(vec2(5.0, 5.0));
        let positions = [vec2(0.0, 0.0), vec2(2.0, 2.0), vec2(10.0, 10.0)];
        let (i, distance) = slime.nearest_position(positions.into_iter()).unwrap();
        println!("distance={}", distance);
        assert_eq!(i, 1);
    }

    #[test]
    fn unique_skills_works() {
        let cases = [
            ((0, 0, 0), 0),
            ((2, 0, 0), 1),
            ((0, 2, 0), 1),
            ((0, 0, 2), 1),
            ((3, 1, 0), 2),
            ((0, 1, 3), 2),
            ((1, 1, 1), 3),
        ];
        for (skills, expected) in cases {
            let skills: Skills = skills.into();
            let res = skills.unique_skills();
            assert_eq!(res, expected, "Failed on: {:?}", skills);
        }
    }

    #[test]
    fn breed_works() {
        let mut a = Slime::create_test(vec2(0.0, 0.0));
        let mut b = Slime::create_test(vec2(0.0, 0.0));
        let child = a.breed(&mut b, 10.0, 0.0);
        assert_eq!(child.skills.count_levels(), 0);
        a.skills.vision = 6;
        let child = a.breed(&mut b, 10.0, 0.0);
        assert_eq!(child.skills.count_levels(), 2);
        a.skills.vision = 6;
        b.skills.jumper = 6;
        let child = a.breed(&mut b, 10.0, 0.0);
        assert_eq!(child.skills.count_levels(), 4);
        a.skills = (6, 6, 6).into();
        b.skills = (3, 3, 3).into();
        let child = a.breed(&mut b, 10.0, 0.0);
        assert_eq!(child.skills.count_levels(), 3);
    }
}
