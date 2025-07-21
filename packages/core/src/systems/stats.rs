use crate::components::world::{Ant, FoodSource, Position};
use crate::engine::stats::Stats;
use hecs::World;

pub fn update_world_stats(world: &mut World, stats: &mut Stats) {
    let ants = world.query::<(&Position, &Ant)>().iter().count();

    let food_sources = world.query::<(&Position, &FoodSource)>().iter().count();

    stats.ants = ants as u32;
    stats.food_sources = food_sources as u32;
}
