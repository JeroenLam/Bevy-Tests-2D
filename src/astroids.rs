use std::ops::Range;
use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets, 
    collission_detection::Collider, 
    movement::{
        Acceleration, 
        MovingObjectBundle, 
        Velocity}
};

const VELOCITY_SCALER: f32 = 5.0;
const ACCELERATION_SCALER: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(
        &self, 
        app: &mut App
    ) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),

        });
        app.add_systems(
            Update,
             (
                spawn_astroids, 
                handle_asteroid_collisions,
                rotate_asteroids
            )
            );
    }
}

fn spawn_astroids(
    mut commands: Commands, 
    mut spawn_timer: ResMut<SpawnTimer>, 
    time: Res<Time>, 
    scene_assets: Res<SceneAssets>
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0., 
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector = 
        || Vec3::new(
            rng.gen_range(-1.0..1.0), 
            0., 
            rng.gen_range(-1.0..1.0)
        ).normalize_or_zero();

    let velocity = random_unit_vector() * VELOCITY_SCALER;
    let acceleration = random_unit_vector() * ACCELERATION_SCALER;

    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(velocity),
        acceleration: Acceleration::new(acceleration),
        collider: Collider::new(RADIUS),
        model: SceneBundle {
            scene: scene_assets.asteroid.clone(),
            transform: Transform::from_translation(translation),
            ..default()
        },
    },
    Asteroid,
    ))
    ;
}

fn rotate_asteroids(
    mut query: Query<&mut Transform, With<Asteroid>>, 
    time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}

fn handle_asteroid_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Collider), With<Asteroid>>,
) {
    for (entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_entities.iter() {
            // Asteroid collided with another asteroid.
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // Despawn the asteroid. If hit by anything else.
            commands.entity(entity).despawn_recursive();
        }
    }
}