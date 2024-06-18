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

fn spawn_map_platforms(mut commands: Commands, terrain_assets: ResMut<TerrainAssets>) {
    // Ground
    spawn_map_platform(
        &mut commands,
        TerrainObject::new(0., -48., 20, 2, TerrainType::GrassGreen),
        &terrain_assets,
    );

    spawn_map_platform(
        &mut commands,
        TerrainObject::new(200., 0., 3, 3, TerrainType::Plant),
        &terrain_assets,
    );

    spawn_map_platform(
        &mut commands,
        TerrainObject::new(-200., 0., 4, 4, TerrainType::GrassOrange),
        &terrain_assets,
    );
}

#[derive(Debug)]
pub struct TerrainObject {
    translation_x: f32,
    translation_y: f32,
    size_x: i32,
    size_y: i32,
    terrain_type: TerrainType,
}

impl TerrainObject {
    pub fn new(
        translation_x: f32,
        translation_y: f32,
        size_x: i32,
        size_y: i32,
        terrain_type: TerrainType,
    ) -> Self {
        Self {
            translation_x,
            translation_y,
            size_x,
            size_y,
            terrain_type,
        }
    }
}

fn spawn_map_platform(
    commands: &mut Commands,
    terrain_object: TerrainObject,
    terrain_assets: &ResMut<TerrainAssets>,
    // materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    if terrain_object.size_x < 2 || terrain_object.size_y < 2 {
        todo!()
    }
    // Create the parent entity
    let parent = commands
        .spawn((
            RigidBody::Fixed,
            Collider::cuboid(
                (terrain_object.size_x * 8) as f32,
                (terrain_object.size_y * 8) as f32,
            ),
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(
                    terrain_object.translation_x,
                    terrain_object.translation_y,
                    0.0,
                )),
                visibility: Visibility::Inherited,
                ..default()
            },
        ))
        .id();

    let atlas_layout = terrain_assets
        .map
        .get(&terrain_object.terrain_type)
        .unwrap();
    let texture = &terrain_assets.image;

    // Spawn children to properly render the block
    for y in 0..terrain_object.size_y {
        for x in 0..terrain_object.size_x {
            let mut idx;
            if x == terrain_object.size_x - 1 {
                idx = 2;
            } else if x == 0 {
                idx = 0;
            } else {
                idx = 1;
            }

            if y == terrain_object.size_y - 1 {
                idx += 0;
            } else if y == 0 {
                idx += 6;
            } else {
                idx += 3;
            }

            // TODO: Implement this correctly
            // TODO: Look at the materials resource for texture colouring
            // TODO: Refactor the platform code to different files
            let trans_x = terrain_object.translation_x + (x * 16) as f32;
            let trans_y = terrain_object.translation_y + (y * 16) as f32;

            let child = commands
                .spawn((SpriteSheetBundle {
                    texture: texture.clone().unwrap(),
                    atlas: TextureAtlas {
                        layout: atlas_layout.clone(),
                        index: idx,
                    },
                    transform: Transform::from_translation(Vec3::new(trans_x, trans_y, 0.0)),
                    ..default()
                },))
                .id();
            commands.entity(parent).push_children(&[child]);
        }
    }
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
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(0., 64.)));
    terrain_assets
        .map
        .insert(TerrainType::Wood, texture_atlas_layouts.add(atlas_layout));

    // Load Plant texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(0., 128.)));
    terrain_assets
        .map
        .insert(TerrainType::Plant, texture_atlas_layouts.add(atlas_layout));

    // Load Grass Green texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(96., 0.)));
    terrain_assets.map.insert(
        TerrainType::GrassGreen,
        texture_atlas_layouts.add(atlas_layout),
    );

    // Load GrassOrange texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(96., 64.)));
    terrain_assets.map.insert(
        TerrainType::GrassOrange,
        texture_atlas_layouts.add(atlas_layout),
    );

    // Load GrassPink texture
    let atlas_layout =
        TextureAtlasLayout::from_grid(Vec2::splat(16.), 3, 3, None, Some(Vec2::new(96., 128.)));
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
