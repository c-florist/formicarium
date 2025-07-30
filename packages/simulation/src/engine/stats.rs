use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Stats {
    pub alive_ants: u32,
    pub dead_ants: u32,
    pub food_sources: u32,
    pub food_in_nest: u32,
}
