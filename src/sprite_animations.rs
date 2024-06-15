use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{character_asset_loader::{AnimationType, PlayerAnimationAssets}, collision::Grounded, player::{Jump, Player, PlayerInput}};

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
    mut player_q: Query<(&mut Handle<Image>, &mut AnimationIndices, &mut TextureAtlas, &mut Sprite, &ActionState<PlayerInput>), With<Player>>,
    player_jump: Query<(Option<&Jump>, &Grounded), With<Player>>,
    animations: Res<PlayerAnimationAssets>,
) {
    let Ok((
        mut texture,
        mut animation_indices,
        mut texture_atlas,
        mut sprite,
        input,
    )) = player_q.get_single_mut() else {return;};

    let left_hold  = input.pressed(&PlayerInput::Left);
    let right_hold = input.pressed(&PlayerInput::Right);
    let left_start  = input.just_pressed(&PlayerInput::Left);
    let right_start = input.just_pressed(&PlayerInput::Right);
    let left_end  = input.just_released(&PlayerInput::Left);
    let right_end = input.just_released(&PlayerInput::Right);

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

    let (has_jump, grounded) = player_jump.single();
    // Check if the player is in the air
    let animation_type = 
        //Jumping if jump
        if has_jump.is_some() {
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