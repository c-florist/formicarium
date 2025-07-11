use hecs::Entity;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, PartialEq)]
pub struct Ant;

#[derive(Debug, PartialEq)]
pub struct Food;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AntState {
    Wandering,
    Foraging,
    ReturningToNest,
}

#[derive(Debug, PartialEq)]
pub struct Target(pub Entity);

#[derive(Debug, PartialEq)]
pub struct Wandering;

#[derive(Debug, PartialEq)]
pub struct CarryingCapacity(pub f32);

#[derive(Debug, PartialEq)]
pub struct Payload(pub f32);
