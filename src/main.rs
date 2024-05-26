mod level;
mod entities;
mod player;
mod camera;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use camera::CameraPlugin;
use entities::EntitiesPlugin;
use level::LevelPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // User defined plugins
        .add_plugins(LevelPlugin)
        .add_plugins(EntitiesPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .run();
}

