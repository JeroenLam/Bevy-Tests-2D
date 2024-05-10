use std::ops::Range;
use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    asset_loader::SceneAssets, 
    collission_detection::{
        Collider, 
        CollisionDamage
    }, 
    health::Health, 
    movement::{
        Acceleration, 
        MovingObjectBundle, 
        Velocity
    }, 
    schedule::InGameSet, spaceship::Spaceship
};

const VELOCITY_RANGE: Range<f32> = 4.0..6.0;
const ACCELERATION_RANGE: Range<f32> = 0.9..1.1;
const SPAWN_RANGE: Range<f32> = 40.0..80.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;
const ROTATE_SPEED: f32 = 2.5;
const RADIUS: f32 = 5.0;
const HEALTH: f32 = 80.0;
const COLLISION_DAMAGE: f32 = 35.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

#[derive(Event, Debug)]
pub struct AsteroidSpawnEvent {
    pub asteroid_trans: Vec3,
    pub ship_trans: Vec3,
}

impl AsteroidSpawnEvent {
    pub fn new(asteroid_trans: Vec3, ship_trans: Vec3) -> Self {
        Self { asteroid_trans, ship_trans }
    }
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(
        &self, 
        app: &mut App
    ) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),

        })
        .add_systems(
            Update,
             (
                spawn_astroids, 
                rotate_asteroids
            )
            .in_set(InGameSet::EntityUpdates)
        )
        .add_event::<AsteroidSpawnEvent>();
    }
}

fn spawn_astroids(
    mut commands: Commands, 
    mut spawn_timer: ResMut<SpawnTimer>, 
    time: Res<Time>, 
    scene_assets: Res<SceneAssets>,
    query: Query<&Transform, With<Spaceship>>,
    mut asteroid_spawn_event_writer: EventWriter<AsteroidSpawnEvent>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let Ok(transform_spaceship) = query.get_single() else {
        return;
    };

    let mut rng = rand::thread_rng();
    let mut random_unit_vector = 
        || Vec3::new(
            rng.gen_range(-1.0..1.0), 
            0., 
            rng.gen_range(-1.0..1.0)
        ).normalize_or_zero();

    let mut rng = rand::thread_rng();
    let translation = random_unit_vector() * rng.gen_range(SPAWN_RANGE) + transform_spaceship.translation;
    let velocity = random_unit_vector() * rng.gen_range(VELOCITY_RANGE);
    let acceleration = random_unit_vector() * rng.gen_range(ACCELERATION_RANGE);

    asteroid_spawn_event_writer.send(AsteroidSpawnEvent::new(translation, transform_spaceship.translation));
    commands.spawn(
        (
            MovingObjectBundle {
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
            Health::new(HEALTH),
            CollisionDamage::new(COLLISION_DAMAGE),
        )
    );
}

fn rotate_asteroids(
    mut query: Query<&mut Transform, With<Asteroid>>, 
    time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_z(ROTATE_SPEED * time.delta_seconds());
    }
}