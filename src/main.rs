use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod asset_loader_player;
mod camera;
mod map_platforms;
mod movement_entity;
mod player;
mod player_input;
mod sprite_animations;

use camera::CameraPlugin;
use map_platforms::MapPlatformsPlugin;
use movement_entity::MovementEntityPlugin;
use player::PlayerPlugin;
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
