use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

use crate::asset_loader_player::{AnimationType, AssetLoaderPlayerPlugin, PlayerAnimationAssets};
use crate::sprite_animations::{AnimationIndices, AnimationTimer};

const MOVE_SPEED: f32 = 200.0;
const JUMP_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_input,
                update_grounded_status,
            ))
            .add_plugins(InputManagerPlugin::<PlayerInput>::default())
            .add_plugins(AssetLoaderPlayerPlugin);
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

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Reflect, Debug)]
enum PlayerInput {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Jump,
    Action,
}

impl PlayerInput {
    pub fn player_one_keyboard() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            // ============ Keyboard ============ 
            // Arrow movement
            (PlayerInput::MoveUp, KeyCode::ArrowUp),
            (PlayerInput::MoveDown, KeyCode::ArrowDown),
            (PlayerInput::MoveLeft, KeyCode::ArrowLeft),
            (PlayerInput::MoveRight, KeyCode::ArrowRight),
            // WASD movement
            (PlayerInput::MoveUp, KeyCode::KeyW),
            (PlayerInput::MoveDown, KeyCode::KeyS),
            (PlayerInput::MoveLeft, KeyCode::KeyA),
            (PlayerInput::MoveRight, KeyCode::KeyD),
            // Jumping 
            (PlayerInput::Jump, KeyCode::Space),
        ]);
        map
    }
    pub fn player_one_controller() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            // ============ Controller ============ 
            // Movement
            (PlayerInput::MoveLeft, GamepadButtonType::DPadLeft),
            (PlayerInput::MoveRight, GamepadButtonType::DPadRight),
            (PlayerInput::MoveUp, GamepadButtonType::DPadUp),
            (PlayerInput::MoveDown, GamepadButtonType::DPadDown),
            // Jumping
            (PlayerInput::Jump, GamepadButtonType::South),
        ]);
        map
    }
}

#[derive(Component)]
pub struct CurrentAnimation(AnimationType);

fn spawn_player(
    mut commands: Commands,
    player_animation_assets: Res<PlayerAnimationAssets>,
) {
    // HashMap<AnimationType, (Handle<TextureAtlasLayout>, Handle<Image>, AnimationIndices)>,
    let (idle_atlas_layout, idle_texture, idle_animation_indices) = player_animation_assets.map.get(&AnimationType::Idle).unwrap();


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
        Collider::cuboid(15.0, 15.0),
        Velocity::default(),
        LockedAxes::ROTATION_LOCKED,
        Grounded::new(false),
        CurrentAnimation(AnimationType::Idle),
        InputManagerBundle {
            input_map: PlayerInput::player_one_keyboard(),
            ..default()
        },
    ));
}

fn player_input(
    mut query: Query<(&mut Velocity, &Grounded, &ActionState<PlayerInput>, &mut TextureAtlas, &mut Handle<Image>, &mut AnimationIndices, &mut CurrentAnimation), With<Player>>,
    player_animation_assets: Res<PlayerAnimationAssets>,
) {
    for (mut q_velocity, q_grounded, q_input, mut q_texture_atlas, mut q_texture, mut q_animation_indices, mut q_current_animation) in query.iter_mut() {
        let mut direction = Vec2::ZERO;
        let mut new_animation = q_current_animation.0.clone();

        if q_input.pressed(&PlayerInput::MoveLeft) {
            direction.x -= 1.0;
            new_animation = AnimationType::Run;
        }

        if q_input.pressed(&PlayerInput::MoveRight) {
            direction.x += 1.0;
            new_animation = AnimationType::Run;
        }

        if direction.x == 0.0 {
            new_animation = AnimationType::Idle;
        }

        q_velocity.linvel.x = direction.x * MOVE_SPEED;

        if q_grounded.is_grounded && q_input.just_pressed(&PlayerInput::Jump) {
            q_velocity.linvel.y = JUMP_SPEED;
            new_animation = AnimationType::Jump;
        }

        // Update the player's animation if it has changed
        if new_animation != q_current_animation.0 {
            if let Some((atlas_handle, texture_handle, indices)) = player_animation_assets.map.get(&new_animation) {
                q_texture_atlas.layout = atlas_handle.clone();
                q_texture = texture_handle.clone();
                *q_animation_indices = *indices;
                q_current_animation.0 = new_animation;
            }
        }
    }
}

fn update_grounded_status(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut query: Query<(Entity, &Transform, &mut Grounded), With<Player>>,
) {
    for (entity, transform, mut grounded) in query.iter_mut() {
        let collider_bottom = transform.translation.y - 15.0; // Adjust based on player size
        let ground_check_distance = 0.1;

        let start_point = Vec2::new(transform.translation.x, collider_bottom);
        let end_point = Vec2::new(transform.translation.x, collider_bottom - ground_check_distance);

        let hit = rapier_context.cast_ray(
            start_point,
            end_point - start_point,
            1.0,
            true,
            QueryFilter::default()
        );

        grounded.is_grounded = hit.is_some();

        // Dereference `grounded` to insert the actual component, not the reference
        commands.entity(entity).insert(*grounded);
    }
}