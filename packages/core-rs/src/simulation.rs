use crate::components::{Position, Velocity, Wandering};
use crate::dto::{AntDto, WorldDto};
use crate::systems::{
    apply_velocity_system, food_discovery_system, state_transition_system, target_movement_system,
    wandering_system,
};
use hecs::{Entity, World};
use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    world: World,
    // We store entities in a vector to have a stable way to reference them from JS.
    entities: Vec<Entity>,
}

fn get_world_state_dto(world: &World) -> WorldDto {
    let ants = world
        .query::<&Position>()
        .iter()
        .map(|(_entity, position)| AntDto {
            x: position.x,
            y: position.y,
        })
        .collect();

    WorldDto {
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
        let mut entities = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..500 {
            let x = 500.0;
            let y = 300.0;
            let dx = rng.gen_range(-1.0..1.0);
            let dy = rng.gen_range(-1.0..1.0);
            let entity = world.spawn((Position { x, y }, Velocity { dx, dy }, Wandering));
            entities.push(entity);
        }

        Self { world, entities }
    }

    pub fn tick(&mut self) {
        food_discovery_system(&mut self.world);
        state_transition_system(&mut self.world);
        target_movement_system(&mut self.world);
        wandering_system(&mut self.world);
        apply_velocity_system(&mut self.world);
    }

    /// Spawns an ant and returns its stable index as a handle.
    pub fn add_ant(&mut self, x: f32, y: f32, dx: f32, dy: f32) -> usize {
        let entity = self.world.spawn((Position { x, y }, Velocity { dx, dy }));
        self.entities.push(entity);
        // Return the index, which is now a stable handle.
        self.entities.len() - 1
    }

    /// Gets a position component using the stable index.
    pub fn get_ant_position_x(&self, entity_index: usize) -> f32 {
        let entity = self.entities[entity_index];
        self.world.get::<&Position>(entity).unwrap().x
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
    fn get_world_state_returns_correct_dto() {
        // 1. Setup
        let mut simulation = Simulation {
            world: World::new(),
            entities: Vec::new(),
        };
        simulation.add_ant(10.0, 10.0, 0.0, 0.0);
        simulation.add_ant(20.0, 20.0, 0.0, 0.0);

        // 2. Action
        let world_dto = get_world_state_dto(&simulation.world);

        // 3. Assertion
        assert_eq!(world_dto.ants.len(), 2);
        assert_eq!(world_dto.ants[0].x, 10.0);
        assert_eq!(world_dto.ants[0].y, 10.0);
        assert_eq!(world_dto.ants[1].x, 20.0);
        assert_eq!(world_dto.ants[1].y, 20.0);
    }

    #[test]
    fn simulation_new_spawns_500_ants() {
        // 1. Action
        let simulation = Simulation::new();

        // 2. Assertion
        assert_eq!(simulation.entities.len(), 500);
        assert_eq!(simulation.world.len(), 500);
    }
}
