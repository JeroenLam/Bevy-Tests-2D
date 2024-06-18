use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MapPlatformsPlugin;

impl Plugin for MapPlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map_platforms);
    }
}

fn spawn_map_platforms(mut commands: Commands) {
    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                custom_size: Some(Vec2::new(800.0, 20.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(400.0, 10.0)
    ));
}
