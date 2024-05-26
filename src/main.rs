mod camera;
mod player;

use bevy::prelude::*;
use camera::OwnCameraPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(OwnCameraPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}