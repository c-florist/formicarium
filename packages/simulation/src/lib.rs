//! The core simulation library for Formicarium.
//!
//! This crate contains the entire simulation engine and is completely decoupled
//! from any UI or platform-specific code. It is designed to be used as a
//! dependency by other crates that provide a frontend, such as a WASM client
//! or a native server.

pub mod components;
pub mod config;
pub mod engine;
pub mod systems;
pub mod utils;

// Re-export the key data structures and functions to provide a clean public API.
pub use components::dto::{StatsDto, WorldDto};
pub use engine::simulation::{Simulation, SimulationOptions};
