use crate::components::world::{
    Ant, AntState, FoodPayload, PheromoneDeposit, PheromoneToFood, Position,
};
use crate::config::GLOBAL_CONFIG;
use hecs::World;
use rand::Rng;

pub fn pheromone_emission_system(world: &mut World, rng: &mut impl Rng) {
    // TODO: Determine strength of pheromones based on distance from food source / to nest

    // TODO: Currently only emits to_food pheromones
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
        if rng.random_bool(GLOBAL_CONFIG.pheromone.emit_chance) {
            world.spawn((
                Position {
                    x: position.x,
                    y: position.y,
                },
                PheromoneDeposit {
                    strength: GLOBAL_CONFIG.pheromone.initial_strength,
                },
                PheromoneToFood,
            ));
        }
    }
}

pub fn pheromone_decay_system(world: &mut World) {
    // TODO: Different pheromones should decay at different rates
    for (_entity, (pheromone, _)) in world.query_mut::<(&mut PheromoneDeposit, &Position)>() {
        pheromone.strength -= GLOBAL_CONFIG.pheromone.decay_amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::world::{
        Ant, AntState, FoodPayload, Nest, PheromoneDeposit, PheromoneToFood,
    };
    use hecs::World;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_pheromone_emission_system_emits_to_food_pheromones() {
        let mut world = World::new();
        let mut rng = StdRng::seed_from_u64(42);
        // Spawn a nest
        world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));
        // Spawn an ant returning to the nest
        world.spawn((
            Position { x: 36.0, y: 48.0 },
            AntState::ReturningToNest,
            FoodPayload(10),
            Ant { health: 100 },
        ));

        // 2. Action
        for _ in 0..50 {
            pheromone_emission_system(&mut world, &mut rng);
        }

        // 3. Assertion
        let mut query = world.query::<(&PheromoneDeposit, &PheromoneToFood, &Position)>();
        let (_entity, (pheromone, _, position)) = query.iter().next().unwrap();

        assert_eq!(pheromone.strength, 100.0);
        assert_eq!(position.x, 36.0);
        assert_eq!(position.y, 48.0);
    }

    #[test]
    fn test_pheromone_decay_system_decrements_pheromone_strength() {
        // 1. Setup
        let mut world = World::new();
        world.spawn((
            Position { x: 0.0, y: 0.0 },
            PheromoneDeposit { strength: 100.0 },
            PheromoneToFood,
        ));

        // 2. Action
        pheromone_decay_system(&mut world);

        // 3. Assertion
        let mut query = world.query::<(&PheromoneDeposit, &PheromoneToFood, &Position)>();
        let (_entity, (pheromone, _, _)) = query.iter().next().unwrap();

        assert_eq!(pheromone.strength, 95.0);
    }
}
