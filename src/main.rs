mod camera;
mod debug;
mod movement;
mod spaceship;
mod astroids;
mod asset_loader;
mod collission_detection;
mod despawn;
mod schedule;
mod state;
mod health;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use astroids::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;
use collission_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            brightness: 800.0,
        })
        .add_plugins((
            DefaultPlugins, 
        ))
        // User defined plugins.
        .add_plugins((
            AssetLoaderPlugin,
            MovementPlugin,
            DebugPlugin,
            SpaceshipPlugin,
            CollisionDetectionPlugin,
            AsteroidPlugin,
            CameraPlugin,
            DespawnPlugin,
            SchedulePlugin,
            StatePlugin,
        ))
        .run();
}