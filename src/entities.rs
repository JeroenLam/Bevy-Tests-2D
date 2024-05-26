use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct EntityBundle {
    pub velocity: Velocity,
    pub collider: Collider,
    pub model: SpriteBundle,    
}

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_balls);
    }
}

// Build the level from multiple ground elements
fn setup_balls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    create_ball(&mut commands, Vec3::new(-100.0,  300.0, 0.0), 30.0, &asset_server);
    create_ball(&mut commands, Vec3::new(-110.0,  400.0, 0.0), 20.0, &asset_server);
    create_cube(&mut commands, Vec3::new(80.0,  -200.0, 0.0), Vec2::splat(16.0), &asset_server);
    create_cube(&mut commands, Vec3::new(130.0,  -200.0, 0.0), Vec2::splat(32.0), &asset_server);
    create_cube(&mut commands, Vec3::new(190.0,  -200.0, 0.0), Vec2::splat(64.0), &asset_server);
    create_cube(&mut commands, Vec3::new(350.0,  -100.0, 0.0), Vec2::splat(128.0), &asset_server);
}

// Helper function to create a ball element
fn create_ball(
   commands: &mut Commands,
   position: Vec3,
   radius: f32,
   asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        EntityBundle {
            collider: Collider::ball(radius),
            velocity: Velocity {
                ..default()
            },
            model: SpriteBundle {
                texture: asset_server.load("burger.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(radius * 2.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            }
        }, 
        RigidBody::Dynamic,
        Restitution::coefficient(0.7),
    ));
}

// Helper function to create a cube element
fn create_cube(
    commands: &mut Commands,
    position: Vec3,
    size: Vec2,
    asset_server: &Res<AssetServer>,
 ) {
     commands.spawn((
         EntityBundle {
             collider: Collider::cuboid(size.x, size.y),
             velocity: Velocity { ..default() },
             model: SpriteBundle {
                 texture: asset_server.load("crate.png"),
                 sprite: Sprite {
                     custom_size: Some(Vec2::new(size.x * 2.0, size.y * 2.0)),
                     ..default()
                 },
                 transform: Transform::from_translation(position),
                 ..default()
             }
         }, 
         RigidBody::Dynamic,
     ));
 }