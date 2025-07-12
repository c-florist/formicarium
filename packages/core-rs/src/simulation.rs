use crate::components::{Ant, Nest, Position, Velocity, Wandering};
use crate::dto::{AntDto, NestDto, WorldDto};
use crate::systems::{
    apply_velocity_system, food_discovery_system, state_transition_system, target_movement_system,
    wandering_system,
};
use hecs::World;
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    world: World,
}

fn get_world_state_dto(world: &World) -> WorldDto {
    let nest = world
        .query::<(&Position, &Nest)>()
        .iter()
        .next()
        .map(|(_, (pos, _))| NestDto { x: pos.x, y: pos.y })
        .expect("World should have a nest");

    let ants = world
        .query::<(&Position, &Ant)>()
        .iter()
        .map(|(_entity, (position, _))| AntDto {
            x: position.x,
            y: position.y,
        })
        .collect();

    WorldDto {
        nest,
        ants,
        width: 1000.0,
        height: 600.0,
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut world = World::new();
        let mut rng = rand::thread_rng();

        world.spawn((Position { x: 0.0, y: 0.0 }, Nest));

        // Spawn ants
        for _ in 0..500 {
            let x = 500.0;
            let y = 300.0;
            let dx = rng.gen_range(-1.0..1.0);
            let dy = rng.gen_range(-1.0..1.0);
            world.spawn((Position { x, y }, Velocity { dx, dy }, Wandering, Ant));
        }

        Self { world }
    }

    pub fn tick(&mut self) {
        // Systems that determine decisions and state changes.
        food_discovery_system(&mut self.world);
        state_transition_system(&mut self.world);

        // Systems that execute movement based on the current state.
        wandering_system(&mut self.world);
        target_movement_system(&mut self.world);

        // Apply calculated velocity to the positions.
        apply_velocity_system(&mut self.world);
    }

    pub fn get_world_state(&self) -> JsValue {
        let dto = get_world_state_dto(&self.world);
        serde_wasm_bindgen::to_value(&dto).unwrap()
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

        // 2. Assertion
        assert_eq!(ants, 500);
        assert_eq!(nests, 1);
    }
}
