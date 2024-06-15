use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

use crate::character_asset_loader::{AnimationType, PlayerAnimationAssets};
use crate::collision::{check_hit, Grounded, HitBox};
use crate::sprite_animations::AnimationTimer;

const MOVE_SPEED: f32 = 100.;
const FALL_SPEED: f32 = 150.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spawn_player)
            .add_systems(Update, move_player)
            .add_systems(Update, player_jump)
            .add_systems(Update, player_fall)
            ;
    }
}

#[derive(Component, Debug)]
pub struct Player {
    has_double_jump: bool,
}

impl Player {
    pub fn new() -> Self {
        Self { has_double_jump: true }
    }
}


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
                error!("Failed to find animation Idle durring startup"); 
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
        Player::new(),
        Grounded(false),
        HitBox(Vec2::splat(32.)),
        InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..default()
        }
    ));
}


#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Reflect, Debug)]
pub enum PlayerInput {
    Left, 
    Right, 
    Jump
}

impl PlayerInput {
    pub fn player_one() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            (PlayerInput::Left, KeyCode::KeyA),
            (PlayerInput::Left, KeyCode::ArrowLeft),
            (PlayerInput::Right, KeyCode::KeyD),
            (PlayerInput::Right, KeyCode::ArrowRight),
            (PlayerInput::Jump, KeyCode::KeyW),
            (PlayerInput::Jump, KeyCode::ArrowUp),
            (PlayerInput::Jump, KeyCode::Space),
        ]);
        map
    }
}


fn move_player(
    mut commands: Commands, 
    mut player: Query<(Entity, &mut Transform, &Grounded, &HitBox, &ActionState<PlayerInput>), With<Player>>,
    hitboxs: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transform, grounded, &p_hitbox, input)) = player.get_single_mut() else {return;};

    // Handle Jumping
    let up_start = input.just_pressed(&PlayerInput::Jump);
    if up_start {
        commands.entity(player).insert(Jump::new(100.));
    }

    // Handle horizontal movement
    let left_hold  = input.pressed(&PlayerInput::Left);
    let right_hold = input.pressed(&PlayerInput::Right);
    
    let movement = if left_hold && right_hold {
        return;
    } else if left_hold {
        -MOVE_SPEED * time.delta_seconds() * (0.8 + (grounded.0 as u16) as f32)
    } else if right_hold {
        MOVE_SPEED * time.delta_seconds() * (0.8 + (grounded.0 as u16) as f32)
    } else {
        return;
    };

    // Update position
    let new_pos = transform.translation + Vec3::X * movement;
    for (&hitbox, offset) in &hitboxs {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {return;}
    }
    transform.translation = new_pos;

}

#[derive(Component, Debug)]
pub struct Jump {
    height: f32,
}

impl Jump {
    pub fn new(height: f32) -> Self {
        Self { height, }
    }
}

fn player_jump(
    mut commands: Commands, 
    time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
) {
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.height);
    jump.height -= jump_power;
    transform.translation.y += jump_power; 
    if jump.height == 0. {
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