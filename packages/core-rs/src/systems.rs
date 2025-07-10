use crate::components::{Position, Velocity};
use hecs::World;

pub fn movement_system(world: &mut World) {
    // Query for all entities that have both a Position and a Velocity component.
    for (_entity, (pos, vel)) in world.query_mut::<(&mut Position, &Velocity)>() {
        pos.x += vel.dx;
        pos.y += vel.dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Position, Velocity};
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
        // This will fail because the position should be 10.0, not 12.5.
        assert_eq!(pos.x, 12.5);
        assert_eq!(pos.y, 8.5);
    }
}
