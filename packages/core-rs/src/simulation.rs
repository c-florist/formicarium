use crate::systems::movement_system;
use hecs::World;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn tick(&mut self) {
        movement_system(&mut self.world);
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
}
