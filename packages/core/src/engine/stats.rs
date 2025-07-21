use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct Stats {
    pub ants: u32,
    pub food_sources: u32,
    pub food_in_nest: u32,
}

impl Stats {
    pub fn default() -> Self {
        Stats {
            ants: 0,
            food_sources: 0,
            food_in_nest: 0,
        }
    }

    pub fn new() -> Self {
        Stats::default()
    }
}
