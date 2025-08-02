use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct WorldConfig {
    pub food_spawn_chance: f64,
    pub food_spawn_min_distance_to_nest: f32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            food_spawn_chance: 0.01,
            food_spawn_min_distance_to_nest: 25.0,
        }
    }
}

#[derive(Debug)]
pub struct AntConfig {
    pub arrival_distance: f32,
    pub food_payload_amount: u32,
    pub discovery_radius: f32,
    pub min_health: u32,
    pub max_health: u32,
    pub death_animation_ticks: u32,
}

impl Default for AntConfig {
    fn default() -> Self {
        Self {
            arrival_distance: 10.0,
            food_payload_amount: 10,
            discovery_radius: 31.6,
            min_health: 500,
            max_health: 1000,
            death_animation_ticks: 30,
        }
    }
}

#[derive(Debug)]
pub struct PheromoneConfig {
    pub emit_chance: f64,
    pub initial_strength: f32,
    pub decay_amount: f32,
    pub detection_radius: f32,
}

impl Default for PheromoneConfig {
    fn default() -> Self {
        Self {
            emit_chance: 0.5,
            initial_strength: 100.0,
            decay_amount: 5.0,
            detection_radius: 20.0,
        }
    }
}

#[derive(Debug)]
pub struct MovementConfig {
    pub wander_probability: f64,
    pub speed: f32,
}

impl Default for MovementConfig {
    fn default() -> Self {
        Self {
            wander_probability: 0.1,
            speed: 3.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct SimulationConfig {
    pub world: WorldConfig,
    pub ant: AntConfig,
    pub pheromone: PheromoneConfig,
    pub movement: MovementConfig,
}

pub static SIM_CONFIG: Lazy<SimulationConfig> = Lazy::new(SimulationConfig::default);
