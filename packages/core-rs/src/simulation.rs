use crate::components::{Ant, AntState, FoodSource, Nest, Position, Velocity};
use crate::dto::{AntDto, FoodSourceDto, NestDto, WorldDto};
use crate::systems::{
    ant_arrival_at_food_system, ant_arrival_at_nest_system, apply_velocity_system,
    despawn_food_system, enforce_bounds_system, food_discovery_system, target_movement_system,
    wandering_system,
};
use hecs::World;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    #[wasm_bindgen(skip)]
    world: World,
    width: f32,
    height: f32,
}

fn get_world_state_dto(simulation: &Simulation) -> Result<WorldDto, &'static str> {
    let nest = simulation
        .world
        .query::<(&Position, &Nest)>()
        .iter()
        .next()
        .map(|(_, (pos, _))| NestDto { x: pos.x, y: pos.y })
        .ok_or("Could not find nest in world")?;

    let ants = simulation
        .world
        .query::<(&Position, &Ant)>()
        .iter()
        .map(|(_entity, (position, _))| AntDto {
            x: position.x,
            y: position.y,
        })
        .collect();

    let food_sources = simulation
        .world
        .query::<(&Position, &FoodSource)>()
        .iter()
        .map(|(_entity, (position, _))| FoodSourceDto {
            x: position.x,
            y: position.y,
        })
        .collect();

    Ok(WorldDto {
        nest,
        food_sources,
        ants,
        width: simulation.width,
        height: simulation.height,
    })
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut world = World::new();
        let mut rng = Pcg64::from_rng(&mut rand::rng());

        let width = 1000.0;
        let height = 600.0;

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
        for _ in 0..10 {
            let mut x;
            let mut y;
            // Loop until a valid position is found
            loop {
                x = rng.random_range(0.0..width);
                y = rng.random_range(0.0..height);
                let distance_sq = (nest_pos_x - x).powi(2) + (nest_pos_y - y).powi(2);
                // Ensure the food source is not too close to the nest
                if distance_sq > 50.0_f32.powi(2) {
                    break;
                }
            }
            world.spawn((Position { x, y }, FoodSource { amount: 100 }));
        }

        // Spawn ants
        for _ in 0..100 {
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
        }
    }

    pub fn tick(&mut self) {
        // Systems that determine decisions and state changes.
        food_discovery_system(&mut self.world);
        ant_arrival_at_food_system(&mut self.world);
        ant_arrival_at_nest_system(&mut self.world);
        despawn_food_system(&mut self.world);

        // Systems that execute movement based on the current state.
        wandering_system(&mut self.world);
        target_movement_system(&mut self.world);

        // Simulation-wide systems.
        apply_velocity_system(&mut self.world);
        enforce_bounds_system(&mut self.world, self.width, self.height);
    }

    pub fn get_world_state(&self) -> Result<JsValue, JsValue> {
        let dto = get_world_state_dto(self).map_err(JsValue::from_str)?;
        serde_wasm_bindgen::to_value(&dto).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Position, Velocity};

    #[test]
    fn simulation_tick_updates_position() {
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
    fn simulation_new_spawns_correct_entities() {
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
        assert_eq!(ants, 100);
        assert_eq!(nests, 1);
        assert_eq!(food_sources, 10);
    }

    #[test]
    fn test_get_world_state_dto_includes_all_entities() {
        // 1. Setup
        let simulation = Simulation::new();

        // 2. Action
        let dto = get_world_state_dto(&simulation).unwrap();

        // 3. Assertion
        assert_eq!(dto.nest, NestDto { x: 475.0, y: 275.0 });
        assert_eq!(dto.food_sources.len(), 10);
        assert_eq!(dto.ants.len(), 100);
    }
}
