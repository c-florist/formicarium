use crate::components::{
    Ant, AntState, FoodSource, PheromoneDeposit, PheromoneToFood, Position, Target, Velocity,
};
use crate::utils::maths::{calculate_attraction_strength, normalise_vector, target_distance_sq};
use hecs::World;
use rand::Rng;

pub fn target_movement_system(world: &mut World) {
    let mut updates = Vec::new();

    for (entity, (pos, target)) in world.query::<(&Position, &Target)>().iter() {
        if let Ok(target_pos) = world.get::<&Position>(target.0) {
            let dir_x = target_pos.x - pos.x;
            let dir_y = target_pos.y - pos.y;
            if let Some((dx, dy)) = normalise_vector(dir_x, dir_y) {
                updates.push((entity, (dx, dy)))
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

fn set_ant_wandering(ant_vel: &mut Velocity, rng: &mut impl Rng) {
    const WANDER_PROBABILITY: f64 = 0.05;

    if rng.random_bool(WANDER_PROBABILITY) {
        let new_dx: f32 = rng.random_range(-1.0..1.0);
        let new_dy: f32 = rng.random_range(-1.0..1.0);
        if let Some((dx, dy)) = normalise_vector(new_dx, new_dy) {
            ant_vel.dx = dx;
            ant_vel.dy = dy;
        }
    }
}

fn steer_ant_towards_position(ant_pos: Position, target_pos: Position, vel: &mut Velocity) {
    let dir_x = target_pos.x - ant_pos.x;
    let dir_y = target_pos.y - ant_pos.y;
    if let Some((dx, dy)) = normalise_vector(dir_x, dir_y) {
        vel.dx = dx;
        vel.dy = dy;
    }
}

fn find_nearest_food_source(ant_pos: Position, food_sources: &[Position]) -> Option<Position> {
    let mut nearest_food: Option<(Position, f32)> = None;

    for food_pos in food_sources {
        let distance_sq = target_distance_sq(ant_pos.x, ant_pos.y, food_pos.x, food_pos.y);
        if let Some((_, best_distance)) = nearest_food {
            if distance_sq < best_distance {
                nearest_food = Some((*food_pos, distance_sq));
            }
        } else {
            nearest_food = Some((*food_pos, distance_sq));
        }
    }

    nearest_food.map(|(pos, _)| pos)
}

pub fn pheromone_following_system(world: &mut World, rng: &mut impl Rng) {
    const PHEROMONE_DETECTION_RANGE_SQ: f32 = 400.0;

    // Get all food sources
    let food_sources: Vec<Position> = world
        .query::<(&Position, &FoodSource)>()
        .iter()
        .map(|(_, (pos, _))| *pos)
        .collect();

    // Get all to_food pheromones
    let to_food_pheromones: Vec<(Position, f32)> = world
        .query::<(&Position, &PheromoneDeposit, &PheromoneToFood)>()
        .iter()
        .map(|(_, (pos, deposit, _))| (*pos, deposit.strength))
        .collect();

    for (_entity, (pos, vel, state, _)) in
        world.query_mut::<(&Position, &mut Velocity, &AntState, &Ant)>()
    {
        if *state == AntState::Wandering {
            let mut best_pheromone: Option<(Position, f32)> = None;

            // Find the strongest and closest pheromone
            for (pheromone_pos, strength) in &to_food_pheromones {
                let distance_sq =
                    target_distance_sq(pos.x, pos.y, pheromone_pos.x, pheromone_pos.y);
                if distance_sq <= PHEROMONE_DETECTION_RANGE_SQ {
                    let attraction = calculate_attraction_strength(distance_sq, *strength);
                    if let Some((_, best_attraction)) = best_pheromone {
                        if attraction > best_attraction {
                            best_pheromone = Some((*pheromone_pos, attraction));
                        }
                    } else {
                        best_pheromone = Some((*pheromone_pos, attraction));
                    }
                }
            }

            if best_pheromone.is_some() {
                // Find the nearest food source and steer toward it
                let nearest_food = find_nearest_food_source(*pos, &food_sources);
                if let Some(food_pos) = nearest_food {
                    steer_ant_towards_position(*pos, food_pos, vel);
                } else {
                    set_ant_wandering(vel, rng);
                }
            } else {
                set_ant_wandering(vel, rng);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{
        FoodSource, PheromoneDeposit, PheromoneToFood, Position, Target, Velocity,
    };
    use hecs::World;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

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
    fn test_pheromone_following_system_no_pheromones_fallback_to_wandering() {
        let mut world = World::new();
        let mut rng = StdRng::seed_from_u64(123);

        let ant_entity = world.spawn((
            Position { x: 0.0, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Wandering,
            Ant,
        ));

        for _ in 0..100 {
            pheromone_following_system(&mut world, &mut rng);
        }

        let vel = world.get::<&Velocity>(ant_entity).unwrap();
        let magnitude = (vel.dx * vel.dx + vel.dy * vel.dy).sqrt();
        assert!(magnitude > 0.0);
    }

    #[test]
    fn test_pheromone_following_system_steers_toward_food() {
        let mut world = World::new();
        let mut rng = StdRng::seed_from_u64(42);

        // Ant very close to a pheromone
        let ant_entity = world.spawn((
            Position { x: 9.9, y: 0.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Wandering,
            Ant,
        ));

        // Food source further away
        world.spawn((Position { x: 20.0, y: 0.0 }, FoodSource { amount: 100 }));

        // Pheromone very close to ant
        world.spawn((
            Position { x: 10.0, y: 0.0 },
            PheromoneDeposit { strength: 50.0 },
            PheromoneToFood,
        ));

        for _ in 0..10 {
            pheromone_following_system(&mut world, &mut rng);
        }

        let vel = world.get::<&Velocity>(ant_entity).unwrap();
        // Should still steer toward food direction, not get stuck on the nearby pheromone
        assert!(vel.dx > 0.0);
        assert_eq!(vel.dy, 0.0);
    }
}
