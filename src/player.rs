use bevy::prelude::*;

use crate::asset_loader::{AnimationType, PlayerAnimationAssets};
use crate::sprite_animations::{AnimationIndices, AnimationTimer};

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
            ;
    }
}

#[derive(Component)]
struct Player;


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
    ));
}



fn move_player(
    mut player: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands, 
) {
    let Ok((player, mut transform)) = player.get_single_mut() else {return;};

    let left_hold  = input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right_hold = input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    
    if left_hold && right_hold {
    } else if left_hold {
        transform.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if right_hold {
        transform.translation.x += MOVE_SPEED * time.delta_seconds();
    }

    let up_start    = input.any_just_pressed([KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::Space]);
    // let down  = input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    if up_start {
        commands.entity(player).insert(Jump(100.));
    }

}


fn change_player_animation(
    mut player_q: Query<(&mut Handle<Image>, &mut AnimationIndices, &mut TextureAtlas, &mut Sprite), With<Player>>,
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

    // If any move keys pressed, set run sprite
    if left_start || right_start {
        let Some((
            layout_handle_new, 
            texture_new, 
            animation_indices_new
            )) = animations.map.get(&AnimationType::Run)
                else { 
                    error!("Failed to find animation: Run"); 
                    return; 
                };
        
        *texture = texture_new.clone();
        *animation_indices = *animation_indices_new;
        *texture_atlas = TextureAtlas {
            layout: layout_handle_new.clone(),
            index: 0,
        };
    }

    // If no move keys pressed, set idle animation
    if (left_end && !right_hold) || (right_end && !left_hold) {
        let Some((
            layout_handle_new, 
            texture_new, 
            animation_indices_new
            )) = animations.map.get(&AnimationType::Idle)
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
}


#[derive(Component)]
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
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>,
    time: Res<Time>,
) {
    let Ok(mut player) = player.get_single_mut() else {return;};
    if player.translation.y > 0.0 {
        player.translation.y -= time.delta_seconds() * FALL_SPEED;
        if player.translation.y < 0.0 {
            player.translation.y = 0.0;
        }
    }
}