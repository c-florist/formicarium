use crate::components::{Position, Velocity};
use crate::dto::{AntDto, WorldDto};
use crate::systems::{
    apply_velocity_system, food_discovery_system, movement_system, state_transition_system,
    wandering_system,
};
use hecs::{Entity, World};
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

    WorldDto { ants }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            world: World::new(),
            entities: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        food_discovery_system(&mut self.world);
        state_transition_system(&mut self.world);
        movement_system(&mut self.world);
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
        let mut simulation = Simulation::new();
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
}
