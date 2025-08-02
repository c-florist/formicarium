use crate::components::world::{
    Ant, AntState, FoodPayload, FoodSource, Nest, Position, Target, Velocity,
};
use crate::engine::config::SIM_CONFIG;
use crate::engine::stats::Stats;
use crate::utils::maths::target_distance_sq;
use hecs::{Entity, World};
use rand::Rng;

pub fn ant_find_food_system(world: &mut World) {
    let mut updates = Vec::new();
    let discovery_radius_sq = SIM_CONFIG.ant.discovery_radius.powi(2);

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

    for (ant_entity, food_entity) in updates {
        world
            .insert(ant_entity, (Target(food_entity), AntState::Foraging))
            .expect("Failed to update ant state in food_discovery_system");
    }
}

pub fn ant_foraging_system(world: &mut World) {
    let mut to_update_to_wandering = Vec::new();
    let mut to_update_to_returning = Vec::new();
    let arrival_distance_sq = SIM_CONFIG.ant.arrival_distance.powi(2);

    let nest_entity = world
        .query::<&Nest>()
        .iter()
        .next()
        .expect("No nest entity found")
        .0;

    let foraging_ants: Vec<(Entity, Position, Entity)> = world
        .query::<(&Position, &AntState, &Target, &Ant)>()
        .iter()
        .filter(|(_, (_, state, _, _))| **state == AntState::Foraging)
        .map(|(e, (p, _, t, _))| (e, *p, t.0))
        .collect();

    for (ant_entity, ant_pos, target_entity) in foraging_ants {
        let food_source_exists_and_is_valid = world
            .get::<&FoodSource>(target_entity)
            .map_or(false, |food| food.amount > 0);

        if !food_source_exists_and_is_valid {
            to_update_to_wandering.push(ant_entity);
            continue;
        }

        let target_pos = *world.get::<&Position>(target_entity).unwrap();
        let distance_sq = target_distance_sq(ant_pos.x, ant_pos.y, target_pos.x, target_pos.y);

        if distance_sq < arrival_distance_sq {
            to_update_to_returning.push((ant_entity, target_entity));
        }
    }

    for (ant_entity, food_entity) in to_update_to_returning {
        if let Ok(food_source) = world.query_one_mut::<&mut FoodSource>(food_entity) {
            if food_source.amount > 0 {
                food_source.amount -= SIM_CONFIG.ant.food_payload_amount;
                if let Ok(state) = world.query_one_mut::<&mut AntState>(ant_entity) {
                    *state = AntState::ReturningToNest;
                }
                world
                    .insert(
                        ant_entity,
                        (
                            FoodPayload(SIM_CONFIG.ant.food_payload_amount),
                            Target(nest_entity),
                        ),
                    )
                    .expect("Failed to update ant state in ant_foraging_system");
            } else {
                to_update_to_wandering.push(ant_entity);
            }
        }
    }

    for entity in to_update_to_wandering {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::Wandering;
        }
        world.remove_one::<Target>(entity).ok();
    }
}

pub fn ant_returning_system(world: &mut World) {
    let mut to_update_to_wandering = Vec::new();
    let mut food_dropped_at_nest: u32 = 0;
    let arrival_distance_sq = SIM_CONFIG.ant.arrival_distance.powi(2);

    let returning_ants: Vec<(Entity, Position, Entity, FoodPayload)> = world
        .query::<(&Position, &AntState, &Target, &Ant, &FoodPayload)>()
        .iter()
        .filter(|(_, (_, state, _, _, _))| **state == AntState::ReturningToNest)
        .map(|(e, (p, _, t, _, fp))| (e, *p, t.0, *fp))
        .collect();

    for (ant_entity, ant_pos, target_entity, food_payload) in returning_ants {
        if let Ok(target_pos) = world.get::<&Position>(target_entity) {
            let distance_sq = target_distance_sq(ant_pos.x, ant_pos.y, target_pos.x, target_pos.y);

            if distance_sq < arrival_distance_sq && world.get::<&Nest>(target_entity).is_ok() {
                to_update_to_wandering.push(ant_entity);
                food_dropped_at_nest += food_payload.0;
            }
        }
    }

    for entity in to_update_to_wandering {
        if let Ok(state) = world.query_one_mut::<&mut AntState>(entity) {
            *state = AntState::Wandering;
        }
        world.remove::<(FoodPayload, Target)>(entity).ok();
    }

    if let Some((_, nest)) = world.query::<&mut Nest>().iter().next() {
        nest.food_store += food_dropped_at_nest;
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
    let mut ants_to_spawn: u32 = 0;
    let mut spawn_pos = Position { x: 0.0, y: 0.0 };
    for (_, (nest_pos, nest)) in world.query_mut::<(&mut Position, &mut Nest)>() {
        if nest.food_store >= 100 {
            ants_to_spawn = nest.food_store / 10;
            spawn_pos = *nest_pos;
            nest.food_store -= ants_to_spawn * 10;
        }
    }

    world.spawn_batch((0..ants_to_spawn).map(|_| {
        (
            spawn_pos,
            Velocity {
                dx: rng.random_range(-1.0..1.0),
                dy: rng.random_range(-1.0..1.0),
            },
            AntState::Wandering,
            Ant {
                health: rng.random_range(SIM_CONFIG.ant.min_health..SIM_CONFIG.ant.max_health),
            },
        )
    }));
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
            *state = AntState::Dying(SIM_CONFIG.ant.death_animation_ticks);
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
    fn test_ant_find_food_system_updates_ant_to_foraging() {
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
        ant_find_food_system(&mut world);

        // 3. Assertion
        let target = world.get::<&Target>(ant_entity).unwrap();
        assert_eq!(target.0, food_entity);

        let state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*state, AntState::Foraging);
    }

    #[test]
    fn test_ant_foraging_system_updates_ant_to_wandering_when_food_is_gone() {
        // 1. Setup
        let mut world = World::new();
        world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 100 }));
        let ant_entity = world.spawn((
            Position { x: 9.9, y: 9.9 },
            AntState::Foraging,
            Target(food_entity),
            Ant { health: 100 },
        ));

        // 2. Action
        world.despawn(food_entity).unwrap();
        ant_foraging_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::Wandering);
        assert!(world.get::<&Target>(ant_entity).is_err());
    }

    #[test]
    fn test_ant_foraging_system_updates_ant_to_wandering_when_food_is_empty() {
        // 1. Setup
        let mut world = World::new();
        world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));
        let food_entity = world.spawn((Position { x: 10.0, y: 10.0 }, FoodSource { amount: 0 }));
        let ant_entity = world.spawn((
            Position { x: 9.9, y: 9.9 },
            AntState::Foraging,
            Target(food_entity),
            Ant { health: 100 },
        ));

        // 2. Action
        ant_foraging_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::Wandering);
        assert!(world.get::<&Target>(ant_entity).is_err());
    }

    #[test]
    fn test_ant_foraging_system_gathers_food_and_returns_to_nest() {
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
        ant_foraging_system(&mut world);

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
    fn test_ant_returning_system_updates_ant_to_wandering_at_nest() {
        // 1. Setup
        let mut world = World::new();
        let nest_entity = world.spawn((Position { x: 0.0, y: 0.0 }, Nest::new()));
        let ant_entity = world.spawn((
            Position { x: 0.1, y: 0.1 },
            AntState::ReturningToNest,
            FoodPayload(10),
            Target(nest_entity),
            Ant { health: 100 },
        ));

        // 2. Action
        ant_returning_system(&mut world);

        // 3. Assertion
        let ant_state = world.get::<&AntState>(ant_entity).unwrap();
        assert_eq!(*ant_state, AntState::Wandering);
        assert!(world.get::<&FoodPayload>(ant_entity).is_err());
        assert!(world.get::<&Target>(ant_entity).is_err());
        assert_eq!(world.get::<&Nest>(nest_entity).unwrap().food_store, 10);
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

        world.spawn((Position { x: 0.0, y: 0.0 }, Nest { food_store: 150 }));

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
