use crate::components::{Food, Position, Target, Velocity, Wandering};
use hecs::{Entity, World};

pub fn movement_system(world: &mut World) {
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

pub fn food_discovery_system(world: &mut World) {
    let mut updates = Vec::new();
    const DISCOVERY_RADIUS_SQUARED: f32 = 100.0;

    // Collect wandering ants' positions and entities
    let wandering_ants: Vec<(Entity, Position)> = world
        .query::<(&Position, &Wandering)>()
        .iter()
        .map(|(e, (p, _))| (e, *p))
        .collect();

    for (ant_entity, ant_pos) in &wandering_ants {
        let mut closest_food: Option<(Entity, f32)> = None;

        for (food_entity, (food_pos, _)) in world.query::<(&Position, &Food)>().iter() {
            let dx = ant_pos.x - food_pos.x;
            let dy = ant_pos.y - food_pos.y;
            let distance_sq = dx * dx + dy * dy;

            if distance_sq < DISCOVERY_RADIUS_SQUARED {
                if let Some((_, closest_dist_sq)) = closest_food {
                    if distance_sq < closest_dist_sq {
                        closest_food = Some((food_entity, distance_sq));
                    }
                } else {
                    closest_food = Some((food_entity, distance_sq));
                }
            }
        }

        if let Some((food_entity, _)) = closest_food {
            updates.push((*ant_entity, food_entity));
        }
    }

    // Apply updates
    for (ant_entity, food_entity) in updates {
        world.insert_one(ant_entity, Target(food_entity)).unwrap();
        world.remove_one::<Wandering>(ant_entity).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{Food, Position, Target, Velocity, Wandering};
    use hecs::World;

    #[test]
    fn test_movement_system_moves_towards_target() {
        // 1. Setup
        let mut world = World::new();
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, Food));
        let ant_entity = world.spawn((
            Position { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Target(food_entity),
        ));

        // 2. Action
        movement_system(&mut world);

        // 3. Assertion
        let vel = world.get::<&Velocity>(ant_entity).unwrap();
        // The velocity should be a unit vector pointing towards the target.
        // For a target at (10, 10) from (0, 0), the vector is (10, 10).
        // The normalized vector is (1/sqrt(2), 1/sqrt(2)) approx (0.707, 0.707)
        assert!((vel.dx - 0.707).abs() < 1e-3);
        assert!((vel.dy - 0.707).abs() < 1e-3);
    }

    #[test]
    fn test_apply_velocity_system() {
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

    #[test]
    fn test_food_discovery_system() {
        // 1. Setup
        let mut world = World::new();
        let ant_entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            Wandering,
        ));
        let food_entity = world.spawn((Position { x: 12.0, y: 12.0 }, Food));

        // 2. Action
        food_discovery_system(&mut world);

        // 3. Assertion
        // The ant should now have a Target component pointing to the food.
        let target = world.get::<&Target>(ant_entity).unwrap();
        assert_eq!(target.0, food_entity);

        // The ant should no longer have the Wandering component.
        assert!(world.get::<&Wandering>(ant_entity).is_err());
    }
}
