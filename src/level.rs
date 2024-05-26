use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level);
    }
}

// Build the level from multiple ground elements
fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    create_cuboid(&mut commands, Vec3::new(0.0,  -300.0, 0.0), Vec2::new(500.0, 16.0), &asset_server);
    create_cuboid(&mut commands, Vec3::new(510.0,  0.0, 0.0), Vec2::new(16.0, 300.0), &asset_server);
    create_cuboid(&mut commands, Vec3::new(-510.0,  0.0, 0.0), Vec2::new(16.0, 300.0), &asset_server);
}

// Helper function to create a ground element
fn create_cuboid(
    commands: &mut Commands,
    position: Vec3,
    half_size: Vec2,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn(Collider::cuboid(half_size.x, half_size.y))
        .insert(SpriteBundle {
            texture: asset_server.load("floor.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(half_size.x * 2.0, half_size.y * 2.0)),
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        });
}