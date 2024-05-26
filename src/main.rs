mod camera;
mod player;
mod asset_loader;
mod sprite_animations;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use camera::OwnCameraPlugin;
use player::PlayerPlugin;
use sprite_animations::SpriteAnimationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OwnCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .run();
}