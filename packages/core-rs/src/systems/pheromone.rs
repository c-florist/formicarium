use crate::components::{Ant, AntState, FoodPayload, Pheromone, PheromoneType, Position};
use hecs::World;
use rand::Rng;

pub fn pheromone_emission_system(world: &mut World, rng: &mut impl Rng) {
    // TODO: Determine strength of pheromones based on distance from food source / to nest
    const EMIT_CHANCE: f64 = 0.25;

    let ants_returning_to_nest: Vec<Position> = world
        .query::<(&Position, &AntState, &FoodPayload, &Ant)>()
        .iter()
        .filter_map(|(_, (pos, &state, payload, _))| {
            if state == AntState::ReturningToNest && payload.0 > 0 {
                Some(*pos)
            } else {
                None
            }
        })
        .collect();

    for position in ants_returning_to_nest {
        if rng.random_bool(EMIT_CHANCE) {
            world.spawn((
                Position {
                    x: position.x,
                    y: position.y,
                },
                Pheromone { strength: 50.0 },
                PheromoneType::ToFood,
            ));
        }
    }
}

pub fn pheromone_decay_system(world: &mut World) {
    // TODO: Different pheromones should decay at different rates
    for (_entity, (pheromone, _)) in world.query_mut::<(&mut Pheromone, &Position)>() {
        pheromone.strength -= 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Ant, AntState, FoodPayload, Nest, Pheromone, PheromoneType};
    use hecs::World;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_pheromone_emission_system_emits_to_food_pheromones() {
        let mut world = World::new();
        let mut rng = StdRng::seed_from_u64(42);
        // Spawn a nest
        world.spawn((Position { x: 0.0, y: 0.0 }, Nest));
        // Spawn an ant returning to the nest
        world.spawn((
            Position { x: 36.0, y: 48.0 },
            AntState::ReturningToNest,
            FoodPayload(10),
            Ant,
        ));

        // 2. Action
        for _ in 0..50 {
            pheromone_emission_system(&mut world, &mut rng);
        }

        // 3. Assertion
        let mut query = world.query::<(&Pheromone, &PheromoneType, &Position)>();
        let (_entity, (pheromone, p_type, position)) = query.iter().next().unwrap();

        assert_eq!(pheromone.strength, 50.0);
        assert_eq!(*p_type, PheromoneType::ToFood);
        assert_eq!(position.x, 36.0);
        assert_eq!(position.y, 48.0);
    }

    #[test]
    fn test_pheromone_decay_system_decrements_pheromone_strength() {
        // 1. Setup
        let mut world = World::new();
        world.spawn((
            Position { x: 0.0, y: 0.0 },
            Pheromone { strength: 100.0 },
            PheromoneType::ToFood,
        ));

        // 2. Action
        pheromone_decay_system(&mut world);

        // 3. Assertion
        let mut query = world.query::<(&Pheromone, &PheromoneType, &Position)>();
        let (_entity, (pheromone, p_type, _)) = query.iter().next().unwrap();

        assert_eq!(*p_type, PheromoneType::ToFood);
        assert_eq!(pheromone.strength, 99.0);
    }
}
