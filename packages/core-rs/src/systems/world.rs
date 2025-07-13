use crate::components::{Ant, AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity};
use hecs::{Entity, World};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

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

pub fn despawn_food_system(world: &mut World) {
    let mut to_despawn = Vec::new();
    for (entity, (food_source, _)) in world.query::<(&FoodSource, &Position)>().iter() {
        if food_source.amount == 0 {
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
    use crate::components::{AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity};
    use hecs::World;

    #[test]
    fn test_despawn_food_system() {
        // 1. Setup
        let mut world = World::new();
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 0 }));

        // 2. Action
        despawn_food_system(&mut world);

        // 3. Assertion
        assert!(world.get::<&FoodSource>(food_entity).is_err());
    }

    #[test]
    fn test_enforce_bounds_system() {
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
}
