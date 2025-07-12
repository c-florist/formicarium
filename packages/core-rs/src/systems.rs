use crate::components::{Ant, AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity};
use hecs::{Entity, World};
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

pub fn wandering_system(world: &mut World) {
    let mut rng = rand::thread_rng();
    const WANDER_PROBABILITY: f64 = 0.05;

    for (_entity, (vel, state, _)) in world.query_mut::<(&mut Velocity, &mut AntState, &Ant)>() {
        if *state == AntState::Wandering && rng.gen_bool(WANDER_PROBABILITY) {
            let new_dx: f32 = rng.gen_range(-1.0..1.0);
            let new_dy: f32 = rng.gen_range(-1.0..1.0);
            let magnitude = (new_dx * new_dx + new_dy * new_dy).sqrt();
            if magnitude > 1e-6 {
                vel.dx = new_dx / magnitude;
                vel.dy = new_dy / magnitude;
            }
        }
    }
}

pub fn food_discovery_system(world: &mut World) {
    let mut updates = Vec::new();
    const DISCOVERY_RADIUS_SQUARED: f32 = 100.0;

    let wandering_ants: Vec<(Entity, Position)> = world
        .query::<(&Position, &AntState, &Ant)>()
        .iter()
        .filter(|&(_, (_, state, _))| *state == AntState::Wandering)
        .map(|(e, (p, _, _))| (e, *p))
        .collect();

    for (ant_entity, ant_pos) in &wandering_ants {
        let mut closest_food: Option<(Entity, f32)> = None;

        for (food_entity, (food_pos, _)) in world.query::<(&Position, &FoodSource)>().iter() {
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
        world
            .insert(ant_entity, (Target(food_entity), AntState::Foraging))
            .unwrap();
    }
}

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

pub fn ant_arrival_at_food_system(world: &mut World) {
    let mut to_update_to_returning = Vec::new();
    let mut to_despawn = Vec::new();
    const ARRIVAL_DISTANCE_SQUARED: f32 = 5.0;
    const FOOD_PAYLOAD_AMOUNT: u32 = 10;

    // Find the nest entity first. Assumes one nest.
    let nest_entity = world
        .query::<&Nest>()
        .iter()
        .next()
        .expect("No nest entity found")
        .0;

    // Collect all ants with targets
    let ants_with_targets: Vec<(Entity, Position, AntState, Entity)> = world
        .query::<(&Position, &AntState, &Target, &Ant)>()
        .iter()
        .map(|(e, (p, s, t, _))| (e, *p, *s, t.0))
        .collect();

    for (ant_entity, ant_pos, ant_state, target_entity) in ants_with_targets {
        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
            let distance_sq =
                (ant_pos.x - target_pos.x).powi(2) + (ant_pos.y - target_pos.y).powi(2);

            if distance_sq < ARRIVAL_DISTANCE_SQUARED {
                // Case 1: Ant is foraging and has reached food.
                if ant_state == AntState::Foraging
                    && world.get::<&FoodSource>(target_entity).is_ok()
                {
                    to_update_to_returning.push((ant_entity, target_entity));
                }
            }
        }
    }

    // Apply updates for ants that have found food
    for (ant_entity, food_entity) in to_update_to_returning {
        if let Ok(food_source) = world.query_one_mut::<&mut FoodSource>(food_entity) {
            food_source.amount -= FOOD_PAYLOAD_AMOUNT;
            if food_source.amount <= 0 {
                to_despawn.push(food_entity);
            }

            if let Ok(state) = world.query_one_mut::<&mut AntState>(ant_entity) {
                *state = AntState::ReturningToNest;
            }
            world
                .insert(
                    ant_entity,
                    (FoodPayload(FOOD_PAYLOAD_AMOUNT), Target(nest_entity)),
                )
                .unwrap();
        }
    }

    for entity in to_despawn {
        world.despawn(entity).unwrap();
    }
}

pub fn ant_arrival_at_nest_system(world: &mut World) {
    let mut to_update_to_wandering = Vec::new();
    const ARRIVAL_DISTANCE_SQUARED: f32 = 5.0;

    // Collect all ants with targets
    let ants_with_targets: Vec<(Entity, Position, AntState, Entity)> = world
        .query::<(&Position, &AntState, &Target, &Ant)>()
        .iter()
        .map(|(e, (p, s, t, _))| (e, *p, *s, t.0))
        .collect();

    for (ant_entity, ant_pos, ant_state, target_entity) in ants_with_targets {
        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
            let distance_sq =
                (ant_pos.x - target_pos.x).powi(2) + (ant_pos.y - target_pos.y).powi(2);

            if distance_sq < ARRIVAL_DISTANCE_SQUARED {
                // Case 2: Ant is returning and has reached the nest.
                if ant_state == AntState::ReturningToNest
                    && world.get::<&Nest>(target_entity).is_ok()
                {
                    to_update_to_wandering.push(ant_entity);
                }
            }
        }
    }

    // Apply updates for ants that have returned to the nest
    for entity in to_update_to_wandering {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::Wandering;
        }
        world.remove::<(FoodPayload, Target)>(entity).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity};
    use hecs::World;

    #[test]
    fn test_ant_arrival_at_nest_system() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest));
        let ant_entity = world.spawn((
            Position { x: 0.0, y: 0.0 },
            AntState::ReturningToNest,
            FoodPayload(10),
            Target(nest_entity),
            Ant,
        ));

        // 2. Action
        ant_arrival_at_nest_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::Wandering);

        // Check that the ant no longer has a payload or target
        assert!(world.get::<&FoodPayload>(ant_entity).is_err());
        assert!(world.get::<&Target>(ant_entity).is_err());
    }

    #[test]
    fn test_ant_arrival_at_food_system() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest));
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 100 }));
        let ant_entity = world.spawn((
            Position { x: 9.9, y: 9.9 },
            AntState::Foraging,
            Target(food_entity),
            Ant,
        ));

        // 2. Action
        ant_arrival_at_food_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::ReturningToNest);

        let food_source = world.get::<&FoodSource>(food_entity).unwrap();
        assert_eq!(food_source.amount, 90);

        let payload = world.get::<&FoodPayload>(ant_entity).unwrap();
        assert_eq!(payload.0, 10);

        let target = world.get::<&Target>(ant_entity).unwrap();
        assert_eq!(target.0, nest_entity);
    }

    #[test]
    fn test_movement_system_moves_towards_target() {
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
        // TODO: Figure out how to test the wandering system when it uses RNG
    }

    #[test]
    fn test_food_discovery_system() {
        // 1. Setup
        let mut world = World::new();
        let ant_entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Wandering,
            Ant,
        ));
        let food_entity = world.spawn((Position { x: 12.0, y: 12.0 }, FoodSource { amount: 100 }));

        // 2. Action
        food_discovery_system(&mut world);

        // 3. Assertion
        // The ant should now have a Target component pointing to the food.
        let target = world.get::<&Target>(ant_entity).unwrap();
        assert_eq!(target.0, food_entity);

        // The ant's state should be updated to Foraging.
        let state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*state, AntState::Foraging);
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
