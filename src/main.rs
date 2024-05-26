mod camera;
mod player;
mod character_asset_loader;
mod sprite_animations;
mod collision;
mod terain_asset_loader;
mod platform;
mod schedule;

use character_asset_loader::CharacterAssetLoaderPlugin;
use bevy::prelude::*;
use camera::OwnCameraPlugin;
use collision::CollisionPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OwnCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CharacterAssetLoaderPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(PlatformPlugin)
        .run();
}