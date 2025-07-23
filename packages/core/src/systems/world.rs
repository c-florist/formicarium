use crate::components::world::{AntState, FoodSource, Nest, PheromoneDeposit, Position, Velocity};
use crate::config::GLOBAL_CONFIG;
use crate::utils::maths::target_distance_sq;
use hecs::World;
use rand::Rng;

pub fn enforce_bounds_system(world: &mut World, width: f32, height: f32) {
    for (_entity, (pos, vel)) in world.query_mut::<(&mut Position, &mut Velocity)>() {
        if pos.x < 0.0 {
            pos.x = 0.0;
            vel.dx = -vel.dx;
        } else if pos.x > width {
            pos.x = width;
            vel.dx = -vel.dx;
        }

        if pos.y < 0.0 {
            pos.y = 0.0;
            vel.dy = -vel.dy;
        } else if pos.y > height {
            pos.y = height;
            vel.dy = -vel.dy;
        }
    }
}

pub fn food_spawn_system(
    world: &mut World,
    world_width: f32,
    world_height: f32,
    max_food_sources: u32,
    rng: &mut impl Rng,
) {
    let nest_pos = world
        .query::<(&Position, &Nest)>()
        .iter()
        .next()
        .map(|(_, (pos, _))| *pos)
        .expect("No nest found when spawing food in food_spawn_system");

    let food_source_count = world.query::<(&Position, &FoodSource)>().iter().count() as u32;

    if food_source_count < max_food_sources
        && rng.random_bool(GLOBAL_CONFIG.world.food_spawn_chance)
    {
        let mut x;
        let mut y;
        loop {
            x = rng.random_range(0.0..world_width);
            y = rng.random_range(0.0..world_height);
            let distance_sq = target_distance_sq(nest_pos.x, nest_pos.y, x, y);
            // Ensure the food source is not too close to the nest
            if distance_sq > GLOBAL_CONFIG.world.food_spawn_min_distance_to_nest.powi(2) {
                break;
            }
        }
        world.spawn((Position { x, y }, FoodSource { amount: 100 }));
    }
}

pub fn despawn_system(world: &mut World) {
    let mut to_despawn = Vec::new();

    for (entity, (_, food_entity, pheromone_entity)) in world
        .query::<(&Position, Option<&FoodSource>, Option<&PheromoneDeposit>)>()
        .iter()
    {
        if let Some(food_source) = food_entity
            && food_source.amount == 0
        {
            to_despawn.push(entity);
        }

        if let Some(pheromone) = pheromone_entity
            && pheromone.strength == 0.0
        {
            to_despawn.push(entity);
        }
    }

    // Find all ants in the Dying state
    for (entity, state) in world.query_mut::<&mut AntState>() {
        if let AntState::Dying(ticks) = state {
            *ticks = ticks.saturating_sub(1);
            if *ticks == 0 {
                to_despawn.push(entity);
            }
        }
    }

    for entity in to_despawn {
        world
            .despawn(entity)
            .expect("Failed to despawn food entity in despawn_food_system");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::world::{
        FoodSource, PheromoneDeposit, PheromoneToFood, Position, Velocity,
    };
    use hecs::World;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_enforce_bounds_system_clamps_bounds_and_inverts_velocity() {
        // 1. Setup
        let mut world = World::new();
        let width = 100.0;
        let height = 100.0;

        // Spawn an ant outside the bounds
        let out_of_bounds_entity = world.spawn((
            Position { x: -10.0, y: 110.0 },
            Velocity { dx: -1.0, dy: 1.0 },
        ));

        // 2. Action
        enforce_bounds_system(&mut world, width, height);

        // 3. Assertion
        let pos = world.get::<&Position>(out_of_bounds_entity).unwrap();
        let vel = world.get::<&Velocity>(out_of_bounds_entity).unwrap();

        // Position should be clamped to the boundaries
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 100.0);

        // Velocity should be inverted
        assert_eq!(vel.dx, 1.0);
        assert_eq!(vel.dy, -1.0);
    }

    #[test]
    fn test_food_spawn_system_spawns_food_at_random_positions() {
        // 1. Setup
        let mut rng = StdRng::seed_from_u64(42);
        let mut world = World::new();

        world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));

        // 2. Action
        for _ in 0..500 {
            food_spawn_system(&mut world, 100.0, 100.0, 100, &mut rng);
        }

        // 3. Assertion
        let food_count = world.query::<(&Position, &FoodSource)>().iter().count();
        assert!(food_count > 1);
    }

    #[test]
    fn test_despawn_system_removes_depleted_food_sources() {
        // 1. Setup
        let mut world = World::new();
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 0 }));

        // 2. Action
        despawn_system(&mut world);

        // 3. Assertion
        assert!(world.get::<&FoodSource>(food_entity).is_err());
    }

    #[test]
    fn test_despawn_system_removes_decayed_pheromones() {
        // 1. Setup
        let mut world = World::new();
        let pheromone_entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            PheromoneDeposit { strength: 0.0 },
            PheromoneToFood,
        ));

        // 2. Action
        despawn_system(&mut world);

        // 3. Assertion
        assert!(world.get::<&PheromoneDeposit>(pheromone_entity).is_err());
    }
}
