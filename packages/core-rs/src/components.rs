use hecs::Entity;
use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq)]
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
pub struct FoodSource;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AntState {
    Foraging,
    ReturningToNest,
}

#[derive(Debug, PartialEq)]
pub struct Target(pub Entity);
