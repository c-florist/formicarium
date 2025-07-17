use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[ts(export, export_to = "../../domain/src/AntDto.ts")]
pub struct AntDto {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub health: u32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[ts(export, export_to = "../../domain/src/NestDto.ts")]
pub struct NestDto {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[ts(export, export_to = "../../domain/src/FoodSourceDto.ts")]
pub struct FoodSourceDto {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../domain/src/StatsDto.ts")]
pub struct StatsDto {
    pub ant_count: u32,
    pub food_source_count: u32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "../../domain/src/WorldDto.ts")]
pub struct WorldDto {
    pub nest: NestDto,
    pub food_sources: Vec<FoodSourceDto>,
    pub ants: Vec<AntDto>,
    pub width: f32,
    pub height: f32,
}
