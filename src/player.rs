use bevy::prelude::*;
use std::collections::HashMap;

const MOVE_SPEED: f32 = 100.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimationAssets>()
            .add_systems(Startup, (load_assets, setup_spawn_player).chain())
            .add_systems(Update, animate_sprite)
            .add_systems(Update, move_player)
            .add_systems(Update, change_player_animation)
            ;
    }
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerAnimationAssets {
    map: HashMap<AnimationType, (Handle<TextureAtlasLayout>, Handle<Image>, AnimationIndices)>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum AnimationType {
    Run,
    Idle,
}

fn load_assets(
    mut player_animation_assets: ResMut<PlayerAnimationAssets>, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // Load Player Idle animations
    let idle_texture = asset_server.load("Main Characters/Mask Dude/Run (32x32).png");
    let idle_atlas_layout = TextureAtlasLayout::from_grid(
            Vec2::splat(32.),
            11, 
            1, 
            None, 
            None
        );
    player_animation_assets.map.insert(AnimationType::Idle, (
            texture_atlas_layouts.add(idle_atlas_layout), 
            idle_texture, 
            AnimationIndices::new(0,10)
        ));

    // Load Player Run animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(
            Vec2::splat(32.),
            12, 
            1, 
            None, 
            None
        );
    player_animation_assets.map.insert(AnimationType::Run, (
            texture_atlas_layouts.add(run_atlas_layout), 
            run_texture, 
            AnimationIndices::new(0,11)
        ));
}

impl FromWorld for PlayerAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let _ = world;
        let map = PlayerAnimationAssets {map: HashMap::new()};
        map
    }
    
}


#[derive(Component, Clone, Copy)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}


#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);


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

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut player = player.single_mut();

    let left  = input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    // let up    = input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    // let down  = input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

    if left {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if right {
        player.translation.x += MOVE_SPEED * time.delta_seconds();
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

    // If any move keys pressed, set run sprite
    if input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyD, KeyCode::ArrowRight]) {
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
    if input.any_just_released([KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyD, KeyCode::ArrowRight])
      && !input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyD, KeyCode::ArrowRight]) {
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

    if input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::KeyD, KeyCode::ArrowRight])
               && !input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        sprite.flip_x = false;
    } else if input.any_just_pressed([KeyCode::KeyA, KeyCode::ArrowLeft])
        && !input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft])
        && input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        sprite.flip_x = false
    }
}