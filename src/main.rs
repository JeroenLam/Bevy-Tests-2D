mod camera;
mod player;
mod character_asset_loader;
mod sprite_animations;
mod collision;
mod terain_asset_loader;
mod platform;

use character_asset_loader::CharacterAssetLoaderPlugin;
use bevy::prelude::*;
use camera::OwnCameraPlugin;
use collision::CollisionPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;
use platform::PlatformPlugin;
use player::{PlayerInput, PlayerPlugin};
use sprite_animations::SpriteAnimationPlugin;

fn main() {
    App::new()
        .add_plugins(InputManagerPlugin::<PlayerInput>::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(OwnCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CharacterAssetLoaderPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(PlatformPlugin)
        .add_plugins(SpriteAnimationPlugin)
        .run();
}