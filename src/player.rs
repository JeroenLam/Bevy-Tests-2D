use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::asset_loader_player::{AnimationType, AssetLoaderPlayerPlugin, PlayerAnimationAssets};
use crate::player_input::{PlayerInput, PlayerInputPlugin};
use crate::sprite_animations::{AnimationIndices, AnimationTimer};

const MOVE_SPEED: f32 = 400.0;
const JUMP_SPEED: f32 = 500.0;
const COLLISION_H2: f32 = 15.0;
const COLLISION_W2: f32 = 10.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_input, update_grounded_status))
            .add_plugins(AssetLoaderPlayerPlugin)
            .add_plugins(PlayerInputPlugin);
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct Grounded {
    is_grounded: bool,
}

impl Grounded {
    pub fn new(is_grounded: bool) -> Self {
        Self { is_grounded }
    }
}

#[derive(Component)]
pub struct CurrentAnimation {
    animation_type: AnimationType,
}

impl CurrentAnimation {
    pub fn new(animation_type: AnimationType) -> Self {
        Self { animation_type }
    }
}

fn spawn_player(mut commands: Commands, player_animation_assets: Res<PlayerAnimationAssets>) {
    // HashMap<AnimationType, (Handle<TextureAtlasLayout>, Handle<Image>, AnimationIndices)>,
    let (idle_atlas_layout, idle_texture, idle_animation_indices) = player_animation_assets
        .map
        .get(&AnimationType::Idle)
        .unwrap();

    commands.spawn((
        SpriteSheetBundle {
            texture: idle_texture.clone(),
            atlas: TextureAtlas {
                layout: idle_atlas_layout.clone(),
                index: 0,
            },
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        *idle_animation_indices,
        Player,
        RigidBody::Dynamic,
        Collider::cuboid(COLLISION_W2, COLLISION_H2),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Grounded::new(false),
        CurrentAnimation::new(AnimationType::Idle),
        InputManagerBundle {
            input_map: PlayerInput::player_one_keyboard(),
            ..default()
        },
    ));
}

fn player_input(
    mut query_movement: Query<(&mut Velocity, &Grounded, &ActionState<PlayerInput>), With<Player>>,
    mut query_texture: Query<
        (
            &mut TextureAtlas,
            &mut Handle<Image>,
            &mut AnimationIndices,
            &mut CurrentAnimation,
            &mut Sprite,
        ),
        With<Player>,
    >,
    player_animation_assets: Res<PlayerAnimationAssets>,
) {
    // Parse player movement variables
    let Ok((mut q_velocity, q_grounded, q_input)) = query_movement.get_single_mut() else {
        return;
    };
    // Parse player texture variables
    let Ok((
        mut q_texture_atlas,
        mut q_texture,
        mut q_animation_indices,
        mut q_current_animation,
        mut q_sprite,
    )) = query_texture.get_single_mut()
    else {
        return;
    };

    let mut direction = Vec2::ZERO;
    let mut new_animation = q_current_animation.animation_type.clone();

    // Process left/right input
    if q_input.pressed(&PlayerInput::MoveLeft) {
        direction.x -= 1.0;
        new_animation = AnimationType::Run;
        q_sprite.flip_x = true;
    }
    if q_input.pressed(&PlayerInput::MoveRight) {
        direction.x += 1.0;
        new_animation = AnimationType::Run;
        q_sprite.flip_x = false;
    }
    if q_input.pressed(&PlayerInput::MoveLeft) && q_input.pressed(&PlayerInput::MoveRight) {
        direction.x = 0.0;
    }
    if direction.x == 0.0 {
        new_animation = AnimationType::Idle;
    }

    // Process sprint input
    if q_input.pressed(&PlayerInput::Run) {
        direction.x *= 1.5;
    }

    // Set horizontal movement
    q_velocity.linvel.x = direction.x * MOVE_SPEED;

    // Set vertical movement
    if q_grounded.is_grounded && q_input.just_pressed(&PlayerInput::Jump) {
        q_velocity.linvel.y = JUMP_SPEED;
    }

    // Check if the player is jumping / falling
    if q_grounded.is_grounded == false {
        if q_velocity.linvel.y > 0.1 {
            new_animation = AnimationType::Jump;
        }
        if q_velocity.linvel.y < -0.1 {
            new_animation = AnimationType::Fall;
        }
    }

    println!("v_y : {}", q_velocity.linvel.y);

    // Update the player's animation if it has changed
    if new_animation != q_current_animation.animation_type {
        if let Some((layout_handle_new, texture_handle_new, animation_indices_new)) =
            player_animation_assets.map.get(&new_animation)
        {
            *q_texture_atlas = TextureAtlas {
                layout: layout_handle_new.clone(),
                index: 0,
            };
            *q_texture = texture_handle_new.clone();
            *q_animation_indices = *animation_indices_new;
            q_current_animation.animation_type = new_animation;
        }
    }
}

fn update_grounded_status(
    rapier_context: Res<RapierContext>,
    mut query: Query<(&Transform, &mut Grounded), With<Player>>,
) {
    for (transform, mut grounded) in query.iter_mut() {
        let start_point = Vec2::new(
            transform.translation.x,
            transform.translation.y - COLLISION_H2 - 0.01,
        );
        let end_point = Vec2::new(
            transform.translation.x,
            transform.translation.y - COLLISION_H2 - 0.1,
        );

        let hit = rapier_context.cast_ray(
            start_point,
            end_point - start_point,
            1.0,
            true,
            QueryFilter::default(),
        );

        grounded.is_grounded = hit.is_some();
    }
}
