use bevy::prelude::*;

use crate::{
    collision::HitBox, terain_asset_loader::{
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
    assets: Res<TerainAssets>,
) {
    let Some((
        layout_handle, 
        texture, 
        _animation_indices
        )) = assets.map.get(&TerainType::Block)
            else { 
                error!("Failed to find Block Texture: Idle"); 
                return; 
            };

    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(0.0, -48.0, 0.0)),
            sprite: Sprite { custom_size: Some(Vec2::new(320., 32.)),
                color: Color::WHITE,
                ..Default::default()
            },
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
            ..Default::default()
        },
        HitBox(Vec2::new(320., 32.)),
    ));
    commands.spawn((
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(100., 25., 0.)),
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
                color: Color::WHITE,
                ..Default::default()
            },
            texture: texture.clone(),
            atlas: TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
            ..Default::default()
        },
        HitBox(Vec2::new(32., 32.)),
    ));
}