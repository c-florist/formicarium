use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimulationConfig {
    pub tick_rate: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorldConfig {
    pub max_food_sources: usize,
    pub food_spawn_chance: f64,
    pub food_spawn_min_distance_to_nest: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AntConfig {
    pub arrival_distance: f32,
    pub food_payload_amount: u32,
    pub discovery_radius: f32,
    pub reproduction_chance: f64,
    pub min_health: u32,
    pub max_health: u32,
    pub death_animation_ticks: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PheromoneConfig {
    pub emit_chance: f64,
    pub initial_strength: f32,
    pub decay_amount: f32,
    pub detection_radius: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MovementConfig {
    pub wander_probability: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RenderingConfig {
    pub animation_speed: u64,
}

#[derive(Deserialize, Debug)]
pub struct GameConfig {
    pub simulation: SimulationConfig,
    pub world: WorldConfig,
    pub ant: AntConfig,
    pub pheromone: PheromoneConfig,
    pub movement: MovementConfig,
    pub rendering: RenderingConfig,
}

pub static CONFIG: Lazy<GameConfig> = Lazy::new(|| {
    let config_str = include_str!("../../domain/src/systemConfig.json");
    serde_json::from_str(config_str).expect("Failed to parse systemConfig.json")
});
