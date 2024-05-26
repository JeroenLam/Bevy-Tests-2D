use bevy::prelude::*;

use crate::{
    collision::HitBox, 
    sprite_animations::AnimationTimer, 
    terain_asset_loader::{
        TerainAssetLoaderPlugin, 
        TerainAssets, 
        TerainType
    }};

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map)
            .add_plugins(TerainAssetLoaderPlugin)
            ;
    }
}


fn spawn_map(
    mut commands: Commands,
    // assets: Res<TerainAssets>,
) {
    // let Some((
    //     layout_handle, 
    //     texture, 
    //     animation_indices
    //     )) = assets.map.get(&TerainType::Block)
    //         else { 
    //             error!("Failed to find Block Texture: Idle"); 
    //             return; 
    //         };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::NEG_Y * 16.),
            sprite: Sprite { custom_size: Some(Vec2::new(200., 5.)),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        HitBox(Vec2::new(200., 5.)),
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(100., 25., 0.)),
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        },
        HitBox(Vec2::new(32., 32.)),
    ));
}