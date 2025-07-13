use crate::components::{Pheromone, Position};
use hecs::World;

pub fn pheromone_decay_system(world: &mut World) {
    // At the moment all pheromone types decay at the same rate
    for (_entity, (pheromone, _)) in world.query_mut::<(&mut Pheromone, &Position)>() {
        pheromone.strength -= 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Pheromone, PheromoneType};
    use hecs::World;

    #[test]
    fn test_pheromone_decay_system_decrements_pheromone_strength() {
        // 1. Setup
        let mut world = World::new();
        let pheromone_entity = world.spawn((
            Pheromone {
                phero_type: PheromoneType::ToFood,
                strength: 100.0,
            },
            Position { x: 0.0, y: 0.0 },
        ));

        // 2. Action
        pheromone_decay_system(&mut world);

        // 3. Assertion
        let pheromone_to_food = world.get::<&Pheromone>(pheromone_entity).unwrap();
        assert_eq!(pheromone_to_food.phero_type, PheromoneType::ToFood);
        assert_eq!(pheromone_to_food.strength, 99.0);
    }
}
