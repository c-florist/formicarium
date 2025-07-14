use crate::components::{Ant, AntState, Position, Target, Velocity};
use hecs::World;
use rand::Rng;

pub fn target_movement_system(world: &mut World) {
    let mut updates = Vec::new();

    for (entity, (pos, target)) in world.query::<(&Position, &Target)>().iter() {
        if let Ok(target_pos) = world.get::<&Position>(target.0) {
            let dir_x = target_pos.x - pos.x;
            let dir_y = target_pos.y - pos.y;
            let magnitude = (dir_x * dir_x + dir_y * dir_y).sqrt();
            if magnitude > 1e-6 {
                updates.push((entity, (dir_x / magnitude, dir_y / magnitude)));
            }
        }
    }

    for (entity, (dx, dy)) in updates {
        if let Ok(vel) = world.query_one_mut::<&mut Velocity>(entity) {
            vel.dx = dx;
            vel.dy = dy;
        }
    }
}

pub fn apply_velocity_system(world: &mut World) {
    for (_entity, (pos, vel)) in world.query_mut::<(&mut Position, &Velocity)>() {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

pub fn wandering_system(world: &mut World, rng: &mut impl Rng) {
    const WANDER_PROBABILITY: f64 = 0.05;

    for (_entity, (vel, state, _)) in world.query_mut::<(&mut Velocity, &mut AntState, &Ant)>() {
        if *state == AntState::Wandering && rng.random_bool(WANDER_PROBABILITY) {
            let new_dx: f32 = rng.random_range(-1.0..1.0);
            let new_dy: f32 = rng.random_range(-1.0..1.0);
            let magnitude = (new_dx * new_dx + new_dy * new_dy).sqrt();
            if magnitude > 1e-6 {
                vel.dx = new_dx / magnitude;
                vel.dy = new_dy / magnitude;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{FoodSource, Position, Target, Velocity};
    use hecs::World;

    #[test]
    fn test_target_movement_system_moves_towards_target() {
        // 1. Setup
        let mut world = World::new();
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 100 }));
        let ant_entity = world.spawn((
            Position { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Target(food_entity),
        ));

        // 2. Action
        target_movement_system(&mut world);

        // 3. Assertion
        let vel = world.get::<&Velocity>(ant_entity).unwrap();
        // The velocity should be a unit vector pointing towards the target.
        // For a target at (10, 10) from (0, 0), the vector is (10, 10).
        // The normalized vector is (1/sqrt(2), 1/sqrt(2)) approx (0.707, 0.707)
        assert!((vel.dx - 0.707).abs() < 1e-3);
        assert!((vel.dy - 0.707).abs() < 1e-3);
    }

    #[test]
    fn test_apply_velocity_system_updates_positions() {
        // 1. Setup
        let mut world = World::new();
        let entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 2.5, dy: -1.5 },
        ));

        // 2. Action
        apply_velocity_system(&mut world);

        // 3. Assertion
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 12.5);
        assert_eq!(pos.y, 8.5);
    }

    #[test]
    fn test_wandering_system() {
        // TODO: Figure out how to test the wandering system when it uses RNG
    }
}
