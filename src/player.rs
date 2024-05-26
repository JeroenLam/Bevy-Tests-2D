use bevy::prelude::*;

use crate::character_asset_loader::{AnimationType, PlayerAnimationAssets};
use crate::collision::{check_hit, Grounded, HitBox};
use crate::sprite_animations::{AnimationIndices, AnimationTimer, SpriteAnimationPlugin};

const MOVE_SPEED: f32 = 100.;
const FALL_SPEED: f32 = 98.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, change_player_animation)
            .add_systems(Update, player_jump)
            .add_systems(Update, player_fall)
            .add_plugins(SpriteAnimationPlugin)
            ;
    }
}

#[derive(Component, Debug)]
pub struct Player;


fn setup_spawn_player(
    mut commands: Commands,
    animations: Res<PlayerAnimationAssets>,
) {
    let Some((
        layout_handle, 
        texture, 
        animation_indices
        )) = animations.map.get(&AnimationType::Idle)
            else { 
                error!("Failed to find animation: Idle"); 
                return; 
            };

    commands.spawn((
        SpriteSheetBundle {
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
            ..default()
        },
        animation_indices.clone(),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player,
        Grounded(false),
        HitBox(Vec2::splat(32.)),
    ));
}



fn move_player(
    mut player: Query<(Entity, &mut Transform, &Grounded, &HitBox), With<Player>>,
    hitboxs: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands, 
) {
    let Ok((player, mut transform, grounded, &p_hitbox)) = player.get_single_mut() else {return;};

    let left_hold  = input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right_hold = input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    
    let movement = if left_hold && right_hold {
        return;
    } else if left_hold {
        -MOVE_SPEED * time.delta_seconds() * (0.5 + (grounded.0 as u16) as f32)
    } else if right_hold {
        MOVE_SPEED * time.delta_seconds() * (0.5 + (grounded.0 as u16) as f32)
    } else {
        return;
    };

    let up_start = input.any_just_pressed([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space]);
    // let down  = input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    if up_start && grounded.0 {
        commands.entity(player).insert(Jump(100.));
        return;
    }

    let new_pos = transform.translation + Vec3::X * movement;
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {return;}
    }
    transform.translation = new_pos;

}


fn change_player_animation(
    mut player_q: Query<(&mut Handle<Image>, &mut AnimationIndices, &mut TextureAtlas, &mut Sprite), With<Player>>,
    player_jump: Query<(Option<&Jump>, &Grounded), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    animations: Res<PlayerAnimationAssets>,
) {
    let (
        mut texture,
        mut animation_indices,
        mut texture_atlas,
        mut sprite,
    ) = player_q.single_mut();

    let left_hold  = input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right_hold = input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let left_start  = input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right_start = input.any_just_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let left_end  = input.any_just_released([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right_end = input.any_just_released([KeyCode::KeyD, KeyCode::ArrowRight]);

    // Flip sprite if needed
    if right_start {
        sprite.flip_x = false;
    } else if left_start {
        sprite.flip_x = true;
    } else if left_end && right_hold {
        sprite.flip_x = false;
    } else if right_end && left_hold {
        sprite.flip_x = true;
    }

    let (jump, grounded) = player_jump.single();
    // Check if the player is in the air
    let animation_type = 
        //Jumping if jump
        if jump.is_some() {
            AnimationType::Jump
        //Falling if no on ground
        } else if !grounded.0 {
            AnimationType::Fall
        // if any move keys pressed set run sprite
        } else if left_start || right_start {
            AnimationType::Run
        } else if (left_end && !right_hold) || (right_end && !left_hold) {
            AnimationType::Idle
        } else {
            AnimationType::Idle
        };

    let Some((
        layout_handle_new, 
        texture_new, 
        animation_indices_new
        )) = animations.map.get(&animation_type)
            else { 
                error!("Failed to find animation: Idle"); 
                return; 
            };

    *texture = texture_new.clone();
    *animation_indices = *animation_indices_new;
    *texture_atlas = TextureAtlas {
        layout: layout_handle_new.clone(),
        index: 0,
    };
    
}


#[derive(Component, Debug)]
struct Jump(f32);

fn player_jump(
    mut commands: Commands, 
    time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
) {
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power; 
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    }
}

fn player_fall(
    mut player: Query<(&mut Transform, &HitBox), (With<Player>, Without<Jump>)>,
    time: Res<Time>,
    hitboxs: Query<(&HitBox, &Transform), Without<Player>>,
) {
    let Ok((mut p_offset, &p_hitbox)) = player.get_single_mut() else {return;};
    let new_pos = p_offset.translation - Vec3::Y * FALL_SPEED * time.delta_seconds();
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {return;}
    }
    p_offset.translation = new_pos;
}