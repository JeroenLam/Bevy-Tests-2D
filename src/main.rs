mod ecs_example;

use bevy::prelude::*;
use ecs_example::HelloPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

