use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AntDto {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct WorldDto {
    pub ants: Vec<AntDto>,
}
