use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod player;
mod movement_entity;
mod map_platforms;
mod camera;
mod sprite_animations;
mod asset_loader_player;

use player::PlayerPlugin;
use movement_entity::MovementEntityPlugin;
use map_platforms::MapPlatformsPlugin;
use camera::CameraPlugin;
use sprite_animations::SpriteAnimationPlugin;

fn main() {
    App::new()
        // Library plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        // User plugins
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementEntityPlugin)
        .add_plugins(MapPlatformsPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .run();
}
