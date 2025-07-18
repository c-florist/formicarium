use crate::components::world::AntState;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, TS)]
#[ts(export, export_to = "../../domain/src/types/AntStateDto.ts")]
#[serde(tag = "type", content = "ticks", rename_all = "camelCase")]
pub enum AntStateDto {
    Wandering,
    Foraging,
    ReturningToNest,
    Dying(u32),
}

impl From<&AntState> for AntStateDto {
    fn from(state: &AntState) -> Self {
        match state {
            AntState::Wandering => AntStateDto::Wandering,
            AntState::Foraging => AntStateDto::Foraging,
            AntState::ReturningToNest => AntStateDto::ReturningToNest,
            AntState::Dying(ticks) => AntStateDto::Dying(*ticks),
        }
    }
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug, Clone)]
#[ts(export, export_to = "../../domain/src/types/AntDto.ts")]
pub struct AntDto {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub state: AntStateDto,
    pub health: u32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug, Clone)]
#[ts(export, export_to = "../../domain/src/types/NestDto.ts")]
pub struct NestDto {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug, Clone)]
#[ts(export, export_to = "../../domain/src/types/FoodSourceDto.ts")]
pub struct FoodSourceDto {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../domain/src/types/StatsDto.ts")]
pub struct StatsDto {
    pub ant_count: u32,
    pub food_source_count: u32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../domain/src/types/WorldDto.ts")]
pub struct WorldDto {
    pub nest: NestDto,
    pub food_sources: Vec<FoodSourceDto>,
    pub ants: Vec<AntDto>,
    pub width: f32,
    pub height: f32,
}
