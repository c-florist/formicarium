use crate::components::dto::{AntDto, FoodSourceDto, NestDto, StatsDto, WorldDto};
use crate::components::world::{Ant, AntState, FoodSource, Nest, Position, Velocity};
use crate::engine::config::SIM_CONFIG;
use crate::engine::stats::Stats;
use crate::systems::{
    ant_arrival_at_food_system, ant_arrival_at_nest_system, ant_dying_system, ant_lifecycle_system,
    apply_velocity_system, despawn_system, enforce_bounds_system, food_discovery_system,
    food_spawn_system, pheromone_decay_system, pheromone_emission_system,
    pheromone_following_system, target_movement_system, update_world_stats,
};
use crate::utils::maths::target_distance_sq;
use hecs::World;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, TS)]
#[ts(export, export_to = "../../domain/src/types/SimulationOptions.ts")]
#[serde(rename_all = "camelCase")]
pub struct SimulationOptions {
    pub width: f32,
    pub height: f32,
    pub starting_ants: u32,
    pub starting_food_sources: u32,
    pub max_food_sources: u32,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        SimulationOptions {
            width: 100.0,
            height: 100.0,
            starting_ants: 50,
            starting_food_sources: 50,
            max_food_sources: 50,
        }
    }
}

pub struct Simulation {
    world: World,
    options: SimulationOptions,
    rng: Pcg64,
    stats: Stats,
}

impl Simulation {
    pub fn new(sim_options: SimulationOptions) -> Self {
        let mut world = World::new();
        let mut rng = Pcg64::from_rng(&mut rand::rng());

        let start_x: f32 = sim_options.width / 2.0;
        let start_y: f32 = sim_options.height / 2.0;

        let nest_pos_x = start_x - 10.0;
        let nest_pos_y = start_y - 10.0;
        world.spawn((
            Position {
                x: nest_pos_x,
                y: nest_pos_y,
            },
            Nest::new(),
        ));

        // Spawn food sources
        for _ in 0..sim_options.starting_food_sources {
            let mut x;
            let mut y;
            // Loop until a valid position is found
            loop {
                x = rng.random_range(0.0..sim_options.width);
                y = rng.random_range(0.0..sim_options.height);
                let distance_sq = target_distance_sq(nest_pos_x, nest_pos_y, x, y);
                // Ensure the food source is not too close to the nest
                if distance_sq > SIM_CONFIG.world.food_spawn_min_distance_to_nest.powi(2) {
                    break;
                }
            }
            world.spawn((Position { x, y }, FoodSource { amount: 100 }));
        }

        // Spawn ants to start
        for _ in 0..sim_options.starting_ants {
            let dx = rng.random_range(-1.0..1.0);
            let dy = rng.random_range(-1.0..1.0);
            let ant_health = rng.random_range(500..1000);
            world.spawn((
                Position {
                    x: start_x,
                    y: start_y,
                },
                Velocity { dx, dy },
                AntState::Wandering,
                Ant { health: ant_health },
            ));
        }

        Self {
            world,
            options: sim_options,
            rng,
            stats: Stats::default(),
        }
    }

    pub fn tick(&mut self) {
        // Systems that control lifecycle events
        ant_lifecycle_system(&mut self.world, &mut self.rng);
        ant_dying_system(&mut self.world, &mut self.stats);
        food_spawn_system(
            &mut self.world,
            self.options.width,
            self.options.height,
            self.options.max_food_sources,
            &mut self.rng,
        );

        // Systems that determine decisions and state changes.
        food_discovery_system(&mut self.world);
        ant_arrival_at_food_system(&mut self.world);
        ant_arrival_at_nest_system(&mut self.world);

        // Pheromone systems that modify the world state.
        pheromone_emission_system(&mut self.world, &mut self.rng);
        pheromone_decay_system(&mut self.world);

        // Clean up systems that remove entities.
        despawn_system(&mut self.world);

        // Systems that execute movement based on the current state.
        pheromone_following_system(&mut self.world, &mut self.rng);
        target_movement_system(&mut self.world);

        // Simulation-wide systems.
        apply_velocity_system(&mut self.world);
        enforce_bounds_system(&mut self.world, self.options.width, self.options.height);
        update_world_stats(&mut self.world, &mut self.stats);
    }

    pub fn get_world_state_dto(&mut self) -> Result<WorldDto, &'static str> {
        let nest = self
            .world
            .query::<(&Position, &Nest)>()
            .iter()
            .next()
            .map(|(_, (pos, _))| NestDto { x: pos.x, y: pos.y })
            .ok_or("Could not find nest in world")?;

        let ants = self
            .world
            .query::<(&Position, &Ant, &AntState)>()
            .iter()
            .map(|(entity, (position, ant, ant_state))| AntDto {
                id: entity.id(),
                x: position.x,
                y: position.y,
                state: ant_state.into(),
                health: ant.health,
            })
            .collect();

        let food_sources = self
            .world
            .query::<(&Position, &FoodSource)>()
            .iter()
            .map(|(entity, (position, food_source))| FoodSourceDto {
                id: entity.id(),
                x: position.x,
                y: position.y,
                amount: food_source.amount,
            })
            .collect();

        Ok(WorldDto {
            nest,
            food_sources,
            ants,
            width: self.options.width,
            height: self.options.height,
        })
    }

    pub fn get_world_statistics_dto(&mut self) -> Result<StatsDto, &'static str> {
        Ok(StatsDto {
            alive_ants: self.stats.alive_ants,
            dead_ants: self.stats.dead_ants,
            food_source_count: self.stats.food_sources,
            food_in_nest: self.stats.food_in_nest,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::world::{Position, Velocity};

    #[test]
    fn test_simulation_tick_updates_position() {
        // 1. Setup
        let params = SimulationOptions::default();
        let mut simulation = Simulation::new(params);
        let entity = simulation.world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 5.0, dy: -5.0 },
        ));

        // 2. Action
        simulation.tick();

        // 3. Assertion
        let position = simulation.world.get::<&Position>(entity).unwrap();
        assert_eq!(position.x, 25.0);
        assert_eq!(position.y, 0.0);
    }

    #[test]
    fn test_simulation_new_spawns_correct_entities() {
        // 1. Action
        let params = SimulationOptions::default();
        let simulation = Simulation::new(params);
        let ants = simulation.world.query::<(&Position, &Ant)>().iter().count();
        let nests = simulation
            .world
            .query::<(&Position, &Nest)>()
            .iter()
            .count();
        let food_sources = simulation
            .world
            .query::<(&Position, &FoodSource)>()
            .iter()
            .count();

        // 2. Assertion
        assert_eq!(ants, 50);
        assert_eq!(nests, 1);
        assert_eq!(food_sources, 50);
    }

    #[test]
    fn test_get_world_state_dto_includes_all_entities_and_dimensions() {
        // 1. Setup
        let params = SimulationOptions::default();
        let mut simulation = Simulation::new(params);

        // 2. Action
        let dto = simulation.get_world_state_dto().unwrap();

        // 3. Assertion
        assert_eq!(dto.width, 100.0);
        assert_eq!(dto.height, 100.0);
        assert_eq!(dto.nest, NestDto { x: 40.0, y: 40.0 });
        assert_eq!(dto.food_sources.len(), 50);
        assert_eq!(dto.ants.len(), 50);
    }

    #[test]
    fn test_get_world_statistics_includes_all_expected_stats() {
        // 1. Setup
        let params = SimulationOptions::default();
        let mut simulation = Simulation::new(params);
        simulation.tick();

        // 2. Action
        let dto = simulation.get_world_statistics_dto().unwrap();

        // 3. Assertion
        assert!(dto.alive_ants >= 50);
        assert!(dto.food_source_count >= 50);
    }
}
