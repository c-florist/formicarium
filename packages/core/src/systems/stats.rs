use crate::components::world::{Ant, AntState, FoodSource, Nest, Position};
use crate::engine::stats::Stats;
use hecs::World;

pub fn update_world_stats(world: &mut World, stats: &mut Stats) {
    let food_sources = world.query::<(&Position, &FoodSource)>().iter().count();
    let alive_ants = world
        .query::<(&Position, &Ant, &AntState)>()
        .iter()
        .filter(|(_, (_, _, state))| !matches!(*state, AntState::Dying(_)))
        .count();

    stats.alive_ants = alive_ants as u32;
    stats.food_sources = food_sources as u32;

    if let Some((_, nest)) = world.query::<&Nest>().iter().next() {
        stats.food_in_nest = nest.food_store;
    }
}
