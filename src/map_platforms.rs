use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

pub struct MapPlatformsPlugin;

impl Plugin for MapPlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map_platforms)
            .init_resource::<TerrainAssets>()
            .add_systems(PreStartup, load_terain_assets);
    }
}

fn spawn_map_platforms(mut commands: Commands) {
    // Ground
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.3, 0.3, 0.7),
                custom_size: Some(Vec2::new(800.0, 20.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(480.0, 8.0),
    ));
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TerrainType {
    StoneBrick,
    Wood,
    Plant,
    GrassGreen,
    GrassOrange,
    GrassPink,
}

#[derive(Resource, Debug)]
pub struct TerrainAssets {
    pub map: HashMap<TerrainType, Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}

fn load_terain_assets(
    mut terrain_assets: ResMut<TerrainAssets>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load terrain texture
    terrain_assets.image = Some(asset_server.load("Terrain/Terrain (16x16).png"));

    // Load Stone Brick texture
    let atlas_layout = TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, None);
    terrain_assets.map.insert(
        TerrainType::StoneBrick,
        texture_atlas_layouts.add(atlas_layout),
    );

    // Load Wood texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(0., 4.)));
    terrain_assets
        .map
        .insert(TerrainType::Wood, texture_atlas_layouts.add(atlas_layout));

    // Load Plant texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(0., 7.)));
    terrain_assets
        .map
        .insert(TerrainType::Plant, texture_atlas_layouts.add(atlas_layout));

    // Load Grass Green texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(6., 0.)));
    terrain_assets.map.insert(
        TerrainType::GrassGreen,
        texture_atlas_layouts.add(atlas_layout),
    );

    // Load GrassOrange texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(6., 4.)));
    terrain_assets.map.insert(
        TerrainType::GrassOrange,
        texture_atlas_layouts.add(atlas_layout),
    );

    // Load GrassPink texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(6., 7.)));
    terrain_assets.map.insert(
        TerrainType::GrassPink,
        texture_atlas_layouts.add(atlas_layout),
    );
}

impl FromWorld for TerrainAssets {
    fn from_world(world: &mut World) -> Self {
        let _ = world;
        let assets = TerrainAssets {
            map: HashMap::new(),
            image: None,
        };
        assets
    }
}
