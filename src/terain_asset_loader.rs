use bevy::prelude::*;
use std::collections::HashMap;

use crate::sprite_animations::AnimationIndices;

pub struct TerainAssetLoaderPlugin;

impl Plugin for TerainAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerainAssets>()
            .add_systems(PreStartup, load_assets)
            ;
    }
}



#[derive(Resource, Debug)]
pub struct TerainAssets {
    pub map: HashMap<TerainType, (Handle<TextureAtlasLayout>, Handle<Image>, AnimationIndices)>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum TerainType {
    Block,
}

fn load_assets(
    mut terain_assets: ResMut<TerainAssets>, 
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // Load Block texture
    let stone_texture = asset_server.load("Terrain/Terrain (16x16).png");
    let stone_atlas_layout = TextureAtlasLayout::from_grid(
            Vec2::splat(48.),
            1, 
            1, 
            None, 
            None
        );
    terain_assets.map.insert(TerainType::Block, (
            texture_atlas_layouts.add(stone_atlas_layout), 
            stone_texture, 
            AnimationIndices::new(0,0)
        ));
}

impl FromWorld for TerainAssets {
    fn from_world(world: &mut World) -> Self {
        let _ = world;
        let map = TerainAssets {map: HashMap::new()};
        map
    }
    
}