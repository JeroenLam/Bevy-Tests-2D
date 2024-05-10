use bevy::prelude::*;

use crate::{astroids::AsteroidSpawnEvent, collission_detection::CollisionEvent, schedule::InGameSet};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LoggerState {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoggerState>()
        .add_systems(
            Update, 
            (
                logger_state_input_events,
                (
                    print_collisions,
                    print_ateroid_spawns
                ).after(InGameSet::EntityUpdates)
                    .run_if(in_state(LoggerState::Info)),
                (
                    print_position
                ).after(InGameSet::EntityUpdates)
                .run_if(in_state(LoggerState::Trace)),
            )
        );
    }
}

pub fn logger_state_input_events(
    mut next_state: ResMut<NextState<LoggerState>>,
    state: Res<State<LoggerState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F9) {
        match state.get() {
            LoggerState::Error => next_state.set(LoggerState::Warn),
            LoggerState::Warn => next_state.set(LoggerState::Info),
            LoggerState::Info => next_state.set(LoggerState::Debug),
            LoggerState::Debug => next_state.set(LoggerState::Trace),
            LoggerState::Trace => next_state.set(LoggerState::Error),
        }
    }
}

fn print_collisions(
    mut collision_event_reader: EventReader<CollisionEvent>,
    transform_query: Query<&Transform>
) {
    for &CollisionEvent {
        entity,
        collided_entity
    } in collision_event_reader.read()
    {
        let Ok(transform_e1) = transform_query.get(entity) else {
            continue;
        };
        let Ok(transform_e2) = transform_query.get(collided_entity) else {
            continue;
        };
        info!(
            "Collision occured between {:?} and {:?}, distance between entities {:?}", 
            entity, 
            collided_entity,
            transform_e1.translation.distance(transform_e2.translation),
        );
    }
}

fn print_ateroid_spawns(
    mut asteroid_spawn_event_reader: EventReader<AsteroidSpawnEvent>,
) {
    for &AsteroidSpawnEvent { asteroid_trans, ship_trans } in asteroid_spawn_event_reader.read()
    {
        
        info!(
            "Asteroid spawned at {:?}, ship is at {:?}, distance to ship {:?}",
            asteroid_trans, 
            ship_trans, 
            asteroid_trans.distance(ship_trans),
        );
    }
}

fn print_position(
    query: Query<(Entity, &Transform)>
) {
    // Log the entity ID and translation of each entity with a `Position` component.
    for (entity, transform) in query.iter() {
        trace!(
            "Entity {:?} is at position {:?},",
            entity, 
            transform.translation
        );
    }
}