use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[ts(export, export_to = "../../domain/src/AntDto.ts")]
pub struct AntDto {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[ts(export, export_to = "../../domain/src/NestDto.ts")]
pub struct NestDto {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, TS, PartialEq, Debug)]
#[ts(export, export_to = "../../domain/src/WorldDto.ts")]
pub struct WorldDto {
    pub nest: NestDto,
    pub ants: Vec<AntDto>,
    pub width: f32,
    pub height: f32,
}
