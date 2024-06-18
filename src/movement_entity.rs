use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MovementEntityPlugin;

impl Plugin for MovementEntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_movement_entities);
    }
}

fn spawn_movement_entities(mut commands: Commands) {
    // Example of another moving entity (could be an enemy or collectible)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(200.0, 50.0, 0.0)),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 10.0),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
    ));
}
