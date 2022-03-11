//! # Evolution Simulation
//! 
//! This simulation consist on 2 interacting entities: food and slime.
//! 
//! ## Food
//! - Main attributes are energy and speed.
//! - Spawns every T time steps.
//! - On spawn it chooses a random energy and direction to move.
//! - Speed its proportional to its energy.
//! - When it detects a slime close by, it changes its direction.
//! - Bigger slimes can be detected more easily (bigger detection radius).
//! - A maximum on M instances of food can exist at the same time.
//! 
//! ## Slime
//! The main evolving creature.
//! - Main attributes are energy and size.
//! - It consume X energy on every time step.
//! - Each movement consumes Y energy (proportional to its size).
//! - When energy is less than a threshold S, its free to move.
//! - By consuming A energy, it can jump to eat.
//! - Every P energy it will evolve.
//! - When meeting another slime, if energy is at least M and the difference
//!   is not less than the smaller slime's energy, a new slime will spawn.
//! - If no food has been consumed on the last W time steps, the slime will die.
//! 
//! ### Slime evolution (skills)
//! - There are 3 evolving paths, the fist time it evolves the slime will randomly
//!   choose a path and will follow it on next evolutions.
//! - A maximum of H skills levels can be hold at the same time.
//! - When a new slime is spawned, if the parents already have some skills it will
//!   inherit them, choosing randomly for each parent and reducing its level by
//!   half (rounding up).
//! - Skill paths:
//!   1) Vision: increase the range of vision to detect food.
//!   2) Efficiency: reduces the energy needed to move around.
//!   3) Jumper: increases the range of its jump.

