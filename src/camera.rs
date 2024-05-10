use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera_pos = Vec3::new(0.0, CAMERA_DISTANCE, 0.0);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(camera_pos)
        .looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}