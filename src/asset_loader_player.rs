use bevy::prelude::*;
use std::collections::HashMap;

use crate::sprite_animations::AnimationIndices;

pub struct AssetLoaderPlayerPlugin;

impl Plugin for AssetLoaderPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimationAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum AnimationType {
    DoubleJump,
    Fall,
    Hit,
    Idle,
    Jump,
    Run,
    WallJump,
}

#[derive(Resource, Debug)]
pub struct PlayerAnimationAssets {
    pub map: HashMap<AnimationType, (Handle<TextureAtlasLayout>, Handle<Image>, AnimationIndices)>,
}

fn load_assets(
    mut player_animation_assets: ResMut<PlayerAnimationAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load Player Idle animations
    let idle_texture = asset_server.load("Main Characters/Mask Dude/Idle (32x32).png");
    let idle_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 11, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::Idle,
        (
            texture_atlas_layouts.add(idle_atlas_layout),
            idle_texture,
            AnimationIndices::new(0, 10),
        ),
    );

    // Load Player Run animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Run (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 12, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::Run,
        (
            texture_atlas_layouts.add(run_atlas_layout),
            run_texture,
            AnimationIndices::new(0, 11),
        ),
    );

    // Load Player Jump animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Jump (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 1, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::Jump,
        (
            texture_atlas_layouts.add(run_atlas_layout),
            run_texture,
            AnimationIndices::new(0, 0),
        ),
    );

    // Load Player Fall animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Fall (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 1, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::Fall,
        (
            texture_atlas_layouts.add(run_atlas_layout),
            run_texture,
            AnimationIndices::new(0, 0),
        ),
    );

    // Load Player Wall Jump animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Wall Jump (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 5, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::WallJump,
        (
            texture_atlas_layouts.add(run_atlas_layout),
            run_texture,
            AnimationIndices::new(0, 4),
        ),
    );

    // Load Player Wall Jump animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Double Jump (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 6, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::DoubleJump,
        (
            texture_atlas_layouts.add(run_atlas_layout),
            run_texture,
            AnimationIndices::new(0, 5),
        ),
    );

    // Load Player Hit animations
    let run_texture = asset_server.load("Main Characters/Mask Dude/Hit (32x32).png");
    let run_atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(32.), 7, 1, None, None);
    player_animation_assets.map.insert(
        AnimationType::Hit,
        (
            texture_atlas_layouts.add(run_atlas_layout),
            run_texture,
            AnimationIndices::new(0, 6),
        ),
    );

    println!("Loaded all player assets!")
}

impl FromWorld for PlayerAnimationAssets {
    fn from_world(world: &mut World) -> Self {
        let _ = world;
        let map = PlayerAnimationAssets {
            map: HashMap::new(),
        };
        map
    }
}
