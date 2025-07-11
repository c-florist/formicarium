use crate::components::{Position, Velocity, Wandering};
use hecs::World;

pub fn movement_system(world: &mut World) {
    // Query for all entities that have both a Position and a Velocity component.
    for (_entity, (pos, vel)) in world.query_mut::<(&mut Position, &Velocity)>() {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

pub fn wandering_system(world: &mut World) {
    for (_entity, (vel, _)) in world.query_mut::<(&mut Velocity, &Wandering)>() {
        // Simple deterministic wandering: slightly alter velocity
        vel.dx += 0.1;
        vel.dy -= 0.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Position, Velocity, Wandering};
    use hecs::World;

    #[test]
    fn test_movement_system() {
        // 1. Setup
        let mut world = World::new();
        let entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 2.5, dy: -1.5 },
        ));

        // 2. Action
        movement_system(&mut world);

        // 3. Assertion
        let pos = world.get::<&Position>(entity).unwrap();
        assert_eq!(pos.x, 12.5);
        assert_eq!(pos.y, 8.5);
    }

    #[test]
    fn test_wandering_system() {
        // 1. Setup
        let mut world = World::new();
        let entity = world.spawn((Velocity { dx: 1.0, dy: 1.0 }, Wandering));

        // 2. Action
        wandering_system(&mut world);

        // 3. Assertion
        let vel = world.get::<&Velocity>(entity).unwrap();
        assert_eq!(vel.dx, 1.1);
        assert_eq!(vel.dy, 0.9);
    }
}
