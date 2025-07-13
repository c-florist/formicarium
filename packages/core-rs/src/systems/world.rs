use crate::components::{FoodSource, Pheromone, Position, Velocity};
use hecs::World;

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

pub fn despawn_system(world: &mut World) {
    let mut to_despawn = Vec::new();
    for (entity, (_, food_entity, pheromone_entity)) in world
        .query::<(&Position, Option<&FoodSource>, Option<&Pheromone>)>()
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

    for entity in to_despawn {
        world
            .despawn(entity)
            .expect("Failed to despawn food entity in despawn_food_system");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{FoodSource, Pheromone, PheromoneType, Position, Velocity};
    use hecs::World;

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
            Pheromone {
                phero_type: PheromoneType::ToFood,
                strength: 0.0,
            },
            Position { x: 10.0, y: 10.0 },
        ));

        // 2. Action
        despawn_system(&mut world);

        // 3. Assertion
        assert!(world.get::<&Pheromone>(pheromone_entity).is_err());
    }
}
