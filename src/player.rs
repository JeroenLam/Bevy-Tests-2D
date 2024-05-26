use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::EntityBundle;

const PLAYER_SIZE_X: f32 = 20.0;
const PLAYER_SPRITE: &str = "player.png";
const PLAYER_SPRITE_X: f32 = 16.0;
const PLAYER_SPRITE_Y: f32 = 28.0;
const SPRITE_Y_SCALE: f32 = PLAYER_SPRITE_Y / PLAYER_SPRITE_X;
const JUMP_FORCE: f32 = 400.0;
const MOVEMENT_SPEED: f32 = 300.0;

#[derive(Component, Debug)]
pub struct Player {
    pub facing_right: bool,
    pub is_colliding: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player)
        .add_systems(Update, player_movement_controls)
        .add_systems(Update, player_animations);
    }
}

fn create_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        EntityBundle {
            collider: Collider::cuboid(PLAYER_SIZE_X, PLAYER_SIZE_X * SPRITE_Y_SCALE),
            velocity: Velocity {
                linvel: Vec2::ZERO,
                angvel: 0.0
            },
            model: SpriteBundle {
                texture: asset_server.load(PLAYER_SPRITE),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(2.0 * PLAYER_SIZE_X, 2.0 * (PLAYER_SIZE_X * SPRITE_Y_SCALE))),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            }
        }, 
        RigidBody::Dynamic,
        Player {
            facing_right: true,
            is_colliding: false,
        },
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn player_movement_controls(
    mut query: Query<(&mut Velocity, &mut Player), With<Player>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut velocity, mut player)) = query.get_single_mut() else {
        info!("No Player entity with Velocity and Player tag exists!");
        return; 
    };

    let up = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    // let down = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    
    if up && player.is_colliding {
        velocity.linvel.y = JUMP_FORCE;
    }
    // if down {
    //     movement = -SPACESHIP_SPEED;
    // } 
    
    // Process horizontal input
    let left = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let right = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    if left {
        player.facing_right = false;
    }
    if right {
        player.facing_right = true;
    } 
    // Determine the movement direction of the player based on pressed keys
    let x_input = -(left as i8) + right as i8;
    let mut player_input_dir = Vec2::new(x_input as f32, 0.0);
    if player_input_dir != Vec2::ZERO {
        player_input_dir /= player_input_dir.length();
    }
    velocity.linvel.x = player_input_dir.x * MOVEMENT_SPEED;

}

fn player_animations(
    mut query: Query<(&mut Sprite, &mut Player), With<Player>>,
) {
    let Ok((mut sprite, player)) = query.get_single_mut() else {
        info!("No Player entity with Sprite and Player tag exists!");
        return; 
    };
    // Determine if Sprite should be flipped or not
    if player.facing_right == true {
        sprite.flip_x = false;
    } else {
        sprite.flip_x = true;
    }
}

fn player_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Player), With<Player>>,
) {
    // Extract Query components
    let Ok((mut player_ent, player_comp)) = query.get_single_mut() else {
        return; 
    };

    // Loop over new collission events
    for collision_event in collision_events.iter() {

    }
}