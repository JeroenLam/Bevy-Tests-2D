use bevy::prelude::*;

use crate::{character_asset_loader::{AnimationType, PlayerAnimationAssets}, collision::Grounded, player::{Jump, Player}};

pub struct SpriteAnimationPlugin;

impl Plugin for SpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite)
            .add_systems(Update, change_player_animation);
    }
}


#[derive(Component, Debug, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl AnimationIndices {
    pub(crate) fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}


#[derive(Component, Debug, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}


fn change_player_animation(
    mut player_q: Query<(&mut Handle<Image>, &mut AnimationIndices, &mut TextureAtlas, &mut Sprite), With<Player>>,
    player_jump: Query<(Option<&Jump>, &Grounded), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    animations: Res<PlayerAnimationAssets>,
) {
    let Ok((
        mut texture,
        mut animation_indices,
        mut texture_atlas,
        mut sprite,
    )) = player_q.get_single_mut() else {return;};

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
            return;
        };

    let Some((
        layout_handle_new, 
        texture_new, 
        animation_indices_new
        )) = animations.map.get(&animation_type)
            else { 
                error!("Failed to find animation durring update: {:?}", animation_type); 
                return; 
            };

    *texture = texture_new.clone();
    *animation_indices = *animation_indices_new;
    *texture_atlas = TextureAtlas {
        layout: layout_handle_new.clone(),
        index: 0,
    };
    
}