use hecs::Entity;

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
pub struct Nest;

#[derive(Debug, PartialEq)]
pub struct FoodSource {
    pub amount: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AntState {
    Wandering,
    Foraging,
    ReturningToNest,
}

#[derive(Debug, PartialEq)]
pub struct Target(pub Entity);

#[derive(Debug, PartialEq)]
pub struct FoodPayload(pub u32);
