use hecs::Entity;

// Movement related components
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
pub struct Target(pub Entity);

// Ant components
#[derive(Debug, PartialEq)]
pub struct Ant;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AntState {
    Wandering,
    Foraging,
    ReturningToNest,
}

#[derive(Debug, PartialEq)]
pub struct FoodPayload(pub u32);

// TODO: Consider making PheromoneTypes their own tag components
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PheromoneType {
    ToFood,
}

#[derive(Debug, PartialEq)]
pub struct Pheromone {
    pub strength: f32,
}

// Static world components
#[derive(Debug, PartialEq)]
pub struct Nest;

#[derive(Debug, PartialEq)]
pub struct FoodSource {
    pub amount: u32,
}
