//! # Evolution Simulation
//!
//! This simulation consist on 2 interacting entities: food and slime.
//!
//! ## Food
#![doc = include_str!("../../docs/food.md")]
//!
//! ## Slime
#![doc = include_str!("../../docs/slime.md")]

pub mod food;
pub mod slime;
pub mod utils;
pub mod world;

pub use world::*;
