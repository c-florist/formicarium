use crate::components::dto::{AntDto, FoodSourceDto, NestDto, WorldDto};
use crate::components::world::{Ant, AntState, FoodSource, Nest, Position, Velocity};
use crate::systems::{
    ant_arrival_at_food_system, ant_arrival_at_nest_system, apply_velocity_system, despawn_system,
    enforce_bounds_system, food_discovery_system, pheromone_decay_system,
    pheromone_emission_system, pheromone_following_system, target_movement_system,
};
use crate::utils::maths::target_distance_sq;
use hecs::World;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

pub struct Simulation {
    world: World,
    width: f32,
    height: f32,
    rng: Pcg64,
}

impl Simulation {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut rng = Pcg64::from_rng(&mut rand::rng());

        let width = 1200.0;
        let height = 800.0;

        const START_X: f32 = 500.0;
        const START_Y: f32 = 300.0;

        let nest_pos_x = START_X - 25.0;
        let nest_pos_y = START_Y - 25.0;
        world.spawn((
            Position {
                x: nest_pos_x,
                y: nest_pos_y,
            },
            Nest,
        ));

        // Spawn food sources
        for _ in 0..15 {
            let mut x;
            let mut y;
            // Loop until a valid position is found
            loop {
                x = rng.random_range(0.0..width);
                y = rng.random_range(0.0..height);
                let distance_sq = target_distance_sq(nest_pos_x, nest_pos_y, x, y);
                // Ensure the food source is not too close to the nest
                if distance_sq > 50.0_f32.powi(2) {
                    break;
                }
            }
            world.spawn((Position { x, y }, FoodSource { amount: 100 }));
        }

        // Spawn ants
        for _ in 0..50 {
            let dx = rng.random_range(-1.0..1.0);
            let dy = rng.random_range(-1.0..1.0);
            world.spawn((
                Position {
                    x: START_X,
                    y: START_Y,
                },
                Velocity { dx, dy },
                AntState::Wandering,
                Ant,
            ));
        }

        Self {
            world,
            width,
            height,
            rng,
        }
    }

    pub fn tick(&mut self) {
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
        enforce_bounds_system(&mut self.world, self.width, self.height);
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
            .query::<(&Position, &Ant)>()
            .iter()
            .map(|(entity, (position, _))| AntDto {
                id: entity.id(),
                x: position.x,
                y: position.y,
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
            width: self.width,
            height: self.height,
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
        let mut simulation = Simulation::new();
        let entity = simulation.world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 5.0, dy: -5.0 },
        ));

        // 2. Action
        simulation.tick();

        // 3. Assertion
        let position = simulation.world.get::<&Position>(entity).unwrap();
        assert_eq!(position.x, 15.0);
        assert_eq!(position.y, 5.0);
    }

    #[test]
    fn test_simulation_new_spawns_correct_entities() {
        // 1. Action
        let simulation = Simulation::new();
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
        assert_eq!(food_sources, 15);
    }

    #[test]
    fn test_get_world_state_dto_includes_all_entities_and_dimensions() {
        // 1. Setup
        let mut simulation = Simulation::new();

        // 2. Action
        let dto = simulation.get_world_state_dto().unwrap();

        // 3. Assertion
        assert_eq!(dto.width, 1200.0);
        assert_eq!(dto.height, 800.0);
        assert_eq!(dto.nest, NestDto { x: 475.0, y: 275.0 });
        assert_eq!(dto.food_sources.len(), 15);
        assert_eq!(dto.ants.len(), 50);
    }
}
