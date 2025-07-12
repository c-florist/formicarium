use crate::components::{AntState, Food, Nest, Payload, Position, Target, Velocity, Wandering};
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

pub fn state_transition_system(world: &mut World) {
    let mut to_update_to_returning = Vec::new();
    let mut to_update_to_wandering = Vec::new();
    const ARRIVAL_DISTANCE_SQUARED: f32 = 0.01;

    // Find the nest entity first. Assumes one nest.
    let nest_entity = match world.query::<&Nest>().iter().next() {
        Some((e, _)) => e,
        None => return,
    };

    // Collect all ants with targets
    let ants_with_targets: Vec<(Entity, Position, AntState, Entity)> = world
        .query::<(&Position, &AntState, &Target)>()
        .iter()
        .map(|(e, (p, s, t))| (e, *p, *s, t.0))
        .collect();

    for (ant_entity, ant_pos, ant_state, target_entity) in ants_with_targets {
        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
            let distance_sq =
                (ant_pos.x - target_pos.x).powi(2) + (ant_pos.y - target_pos.y).powi(2);

            if distance_sq < ARRIVAL_DISTANCE_SQUARED {
                // Case 1: Ant is foraging and has reached food.
                if ant_state == AntState::Foraging && world.get::<&Food>(target_entity).is_ok() {
                    to_update_to_returning.push(ant_entity);
                }
                // Case 2: Ant is returning and has reached the nest.
                else if ant_state == AntState::ReturningToNest
                    && world.get::<&Nest>(target_entity).is_ok()
                {
                    to_update_to_wandering.push(ant_entity);
                }
            }
        }
    }

    // Apply updates for ants that have found food
    for entity in to_update_to_returning {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::ReturningToNest;
        }
        world
            .insert(entity, (Payload(10.0), Target(nest_entity)))
            .unwrap();
    }

    // Apply updates for ants that have returned to the nest
    for entity in to_update_to_wandering {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::Wandering;
        }
        world.remove::<(Payload, Target)>(entity).unwrap();
        world.insert_one(entity, Wandering).unwrap();
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
    let mut rng = rand::thread_rng();
    const WANDER_PROBABILITY: f64 = 0.1;

    for (_entity, (vel, _)) in world.query_mut::<(&mut Velocity, &Wandering)>() {
        if rng.gen_bool(WANDER_PROBABILITY) {
            let new_dx = rng.gen_range(-1.0..1.0);
            let new_dy = rng.gen_range(-1.0..1.0);
            let magnitude: f32 = new_dx * new_dx + new_dy * new_dy;
            let magnitude = magnitude.sqrt();
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
    use crate::components::{AntState, Food, Nest, Payload, Position, Target, Velocity, Wandering};
    use hecs::World;

    #[test]
    fn test_state_transition_system_foraging_to_returning() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest));
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, Food));
        let ant_entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            AntState::Foraging,
            Target(food_entity),
        ));

        // 2. Action
        state_transition_system(&mut world);

        // 3. Assertion
        let ant = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant, AntState::ReturningToNest);

        // Check that the ant now has a payload
        assert!(world.get::<&Payload>(ant_entity).is_ok());

        // Check that the ant is now targeting the nest
        let target = world.get::<&Target>(ant_entity).unwrap();
        assert_eq!(target.0, nest_entity);
    }

    #[test]
    fn test_state_transition_system_returning_to_wandering() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest));
        let ant_entity = world.spawn((
            Position { x: 0.0, y: 0.0 },
            AntState::ReturningToNest,
            Payload(10.0),
            Target(nest_entity),
        ));

        // 2. Action
        state_transition_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::Wandering);

        // Check that the ant no longer has a payload or target
        assert!(world.get::<&Payload>(ant_entity).is_err());
        assert!(world.get::<&Target>(ant_entity).is_err());
        // Check that the ant is now wandering
        assert!(world.get::<&Wandering>(ant_entity).is_ok());
    }

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
        // Figure out how to test the wandering system when it uses RNG
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
