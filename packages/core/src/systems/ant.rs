use crate::components::world::{
    Ant, AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity,
};
use crate::config::CONFIG;
use crate::engine::stats::Stats;
use crate::utils::maths::target_distance_sq;
use hecs::{Entity, World};
use rand::Rng;
use rand::seq::IndexedRandom;

pub fn ant_arrival_at_food_system(world: &mut World) {
    let mut to_update_to_returning = Vec::new();
    let arrival_distance_sq = CONFIG.ant.arrival_distance.powi(2);

    // Find the nest entity first, assumes one nest
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
            let distance_sq = target_distance_sq(ant_pos.x, ant_pos.y, target_pos.x, target_pos.y);

            if distance_sq < arrival_distance_sq
                && ant_state == AntState::Foraging
                && world.get::<&FoodSource>(target_entity).is_ok()
            {
                to_update_to_returning.push((ant_entity, target_entity));
            }
        }
    }

    // Apply updates for ants that have found food
    for (ant_entity, food_entity) in to_update_to_returning {
        if let Ok(food_source) = world.query_one_mut::<&mut FoodSource>(food_entity) {
            if food_source.amount > 0 {
                food_source.amount -= CONFIG.ant.food_payload_amount;
            }

            if let Ok(state) = world.query_one_mut::<&mut AntState>(ant_entity) {
                *state = AntState::ReturningToNest;
            }
            world
                .insert(
                    ant_entity,
                    (
                        FoodPayload(CONFIG.ant.food_payload_amount),
                        Target(nest_entity),
                    ),
                )
                .expect("Failed to update ant state in ant_arrival_at_food_system");
        }
    }
}

pub fn ant_arrival_at_nest_system(world: &mut World) {
    let mut to_update_to_wandering = Vec::new();
    let mut food_dropped_at_nest: u32 = 0;
    let arrival_distance_sq = CONFIG.ant.arrival_distance.powi(2);

    // Collect all ants with targets
    let ants_with_targets: Vec<(Entity, Position, AntState, Entity, FoodPayload)> = world
        .query::<(&Position, &AntState, &Target, &Ant, &FoodPayload)>()
        .iter()
        .map(|(e, (p, s, t, _, fp))| (e, *p, *s, t.0, *fp))
        .collect();

    for (ant_entity, ant_pos, ant_state, target_entity, food_payload) in ants_with_targets {
        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
            let distance_sq = target_distance_sq(ant_pos.x, ant_pos.y, target_pos.x, target_pos.y);

            if distance_sq < arrival_distance_sq
                && ant_state == AntState::ReturningToNest
                && world.get::<&Nest>(target_entity).is_ok()
            {
                to_update_to_wandering.push(ant_entity);
                food_dropped_at_nest += food_payload.0;
            }
        }
    }

    // Apply updates for ants that have returned to the nest
    for entity in to_update_to_wandering {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::Wandering;
        }

        world
            .remove::<(FoodPayload, Target)>(entity)
            .expect("Failed to update ant state in ant_arrival_at_nest_system");
    }

    // Update food store in nest
    if let Some((_, nest)) = world.query::<&mut Nest>().iter().next() {
        nest.food_store += food_dropped_at_nest;
    }
}

pub fn food_discovery_system(world: &mut World) {
    let mut updates = Vec::new();
    let discovery_radius_sq = CONFIG.ant.discovery_radius.powi(2);

    let wandering_ants: Vec<(Entity, Position)> = world
        .query::<(&Position, &AntState, &Ant)>()
        .iter()
        .filter(|&(_, (_, state, _))| *state == AntState::Wandering)
        .map(|(e, (p, _, _))| (e, *p))
        .collect();

    for (ant_entity, ant_pos) in &wandering_ants {
        let mut closest_food: Option<(Entity, f32)> = None;

        for (food_entity, (food_pos, _)) in world.query::<(&Position, &FoodSource)>().iter() {
            let distance_sq = target_distance_sq(ant_pos.x, ant_pos.y, food_pos.x, food_pos.y);

            if distance_sq < discovery_radius_sq {
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
            .expect("Failed to update ant state in food_discovery_system");
    }
}

pub fn ant_lifecycle_system(world: &mut World, rng: &mut impl Rng) {
    // Decrease health of all ants
    for (_, ant) in world.query_mut::<&mut Ant>() {
        if ant.health > 0 {
            ant.health -= 1;
        }
    }

    // Spawn ants when food store reaches threshold
    let mut ants_to_spawn = Vec::new();
    for (_, (nest_pos, nest)) in world.query_mut::<(&mut Position, &mut Nest)>() {
        if nest.food_store >= 1000 {
            let num_ants = nest.food_store / 100;
            for _ in 0..num_ants {
                let dx = rng.random_range(-1.0..1.0);
                let dy = rng.random_range(-1.0..1.0);
                let ant_health = rng.random_range(CONFIG.ant.min_health..CONFIG.ant.max_health);

                ants_to_spawn.push((
                    *nest_pos,
                    Velocity { dx, dy },
                    AntState::Wandering,
                    Ant { health: ant_health },
                ));
            }
        }
    }

    if !ants_to_spawn.is_empty() {
        world.spawn_batch(ants_to_spawn);
    }
}

pub fn ant_dying_system(world: &mut World, stats: &mut Stats) {
    let mut to_update = Vec::new();

    // Find all ants with 0 health that are not already dying
    for (entity, (ant, state)) in world.query::<(&Ant, &AntState)>().iter() {
        if ant.health == 0 {
            if let AntState::Dying(_) = state {
                // Already dying, do nothing
            } else {
                to_update.push(entity);
            }
        }
    }

    // Set state to Dying with a countdown timer
    for entity in to_update {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::Dying(CONFIG.ant.death_animation_ticks);
        }
        stats.dead_ants += 1;
        world.remove_one::<Target>(entity).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::world::{
        AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity,
    };
    use hecs::World;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_ant_arrival_at_nest_system_updates_ant_components() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));
        let ant_entity = world.spawn((
            Position { x: 0.0, y: 0.0 },
            AntState::ReturningToNest,
            FoodPayload(10),
            Target(nest_entity),
            Ant { health: 100 },
        ));

        // 2. Action
        ant_arrival_at_nest_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::Wandering);

        // Check that the ant no longer has a payload or target
        assert!(world.get::<&FoodPayload>(ant_entity).is_err());
        assert!(world.get::<&Target>(ant_entity).is_err());

        // Check that the stats have been updated
        assert_eq!(world.get::<&Nest>(nest_entity).unwrap().food_store, 10);
    }

    #[test]
    fn test_ant_arrival_at_food_system_updates_ant_components_and_depletes_food_source() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 100 }));
        let ant_entity = world.spawn((
            Position { x: 9.9, y: 9.9 },
            AntState::Foraging,
            Target(food_entity),
            Ant { health: 100 },
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
    fn test_food_discovery_system_updates_ant_components() {
        // 1. Setup
        let mut world = World::new();
        let ant_entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Wandering,
            Ant { health: 100 },
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
    fn test_ant_lifecycle_system_decreases_health_of_all_ants() {
        // 1. Setup
        let mut rng = StdRng::seed_from_u64(42);
        let mut world = World::new();
        let ant_entity = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Wandering,
            Ant { health: 100 },
        ));
        let ant_entity2 = world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Foraging,
            Ant { health: 100 },
        ));

        // 2. Action
        ant_lifecycle_system(&mut world, &mut rng);

        // 3. Assertion
        let ant = world.get::<&Ant>(ant_entity).unwrap();
        assert_eq!(ant.health, 99);

        let ant2 = world.get::<&Ant>(ant_entity2).unwrap();
        assert_eq!(ant2.health, 99);
    }

    #[test]
    fn test_ant_lifecycle_system_spawns_new_ants() {
        // 1. Setup
        let mut rng = StdRng::seed_from_u64(42);
        let mut world = World::new();

        world.spawn((Position { x: 0.0, y: 0.0 }, Nest { food_store: 1500 }));

        world.spawn((
            Position { x: 10.0, y: 10.0 },
            Velocity { dx: 0.0, dy: 0.0 },
            AntState::Wandering,
            Ant { health: 100 },
        ));

        // 2. Action
        ant_lifecycle_system(&mut world, &mut rng);

        // 3. Assertion
        let ant_count = world.query::<(&Position, &Ant)>().iter().count();
        assert_eq!(ant_count, 16);
    }
}
