use std::io::BufReader;
use std::{collections::HashMap, path::Path};

use bevy::prelude::Vec2;
use bevy::{
    asset::{AssetLoader, AssetPath, LoadedAsset},
    log,
    prelude::{
        AddAsset, Added, AssetEvent, Assets, BuildChildren, Bundle, Commands, Component,
        DespawnRecursiveExt, Entity, EventReader, GlobalTransform, Handle, Image, Plugin, Query,
        Res, ResMut, Transform, Vec3,
    },
    reflect::TypeUuid,
    transform::TransformBundle,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

use anyhow::Result;
use tiled::{Object, ObjectShape, PropertyValue};

use crate::resources::{SignData, SignsPool, TilesProperties, NpcPool, NpcData};

#[derive(Default)]
pub struct TiledMapPlugin;

impl Plugin for TiledMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<TiledMap>()
            .add_asset_loader(TiledLoader)
            .add_system(process_loaded_maps);
    }
}

#[derive(TypeUuid)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct TiledMap {
    pub map: tiled::Map,

    pub tilemap_textures: HashMap<usize, TilemapTexture>,

    // The offset into the tileset_images for each tile id within each tileset.
    pub tile_image_offsets: HashMap<(usize, tiled::TileId), u32>,
}

// Stores a list of tiled layers.
#[derive(Component, Default)]
pub struct TiledLayersStorage {
    pub storage: HashMap<u32, Entity>,
}

#[derive(Component, Default)]
pub struct TiledMapBundleMarker;

#[derive(Default, Bundle)]
pub struct TiledMapBundle {
    pub tiled_map: Handle<TiledMap>,
    pub storage: TiledLayersStorage,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub marker: TiledMapBundleMarker,
}

pub struct TiledLoader;

impl AssetLoader for TiledLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            // The load context path is the TMX file itself. If the file is at the root of the
            // assets/ directory structure then the tmx_dir will be empty, which is fine.
            let tmx_dir = load_context
                .path()
                .parent()
                .expect("The asset load context was empty.");

            // We need to give the assets path to the Tiled loader as it doesn't know about bevy
            let path = Path::new("assets").join(load_context.path());

            let mut loader = tiled::Loader::new();
            let map = loader
                .load_tmx_map_from(BufReader::new(bytes), path)
                .map_err(|e| anyhow::anyhow!("Could not load TMX map: {e}"))?;

            let mut dependencies = Vec::new();
            let mut tilemap_textures = HashMap::default();
            let mut tile_image_offsets = HashMap::default();

            for (tileset_index, tileset) in map.tilesets().iter().enumerate() {
                let tilemap_texture = match &tileset.image {
                    None => {
                        let mut tile_images: Vec<Handle<Image>> = Vec::new();
                        for (tile_id, tile) in tileset.tiles() {
                            if let Some(img) = &tile.image {
                                let tile_path = tmx_dir.join(&img.source);
                                let asset_path = AssetPath::new(tile_path, None);
                                log::info!("Loading tile image from {asset_path:?} as image ({tileset_index}, {tile_id})");
                                let texture: Handle<Image> =
                                    load_context.get_handle(asset_path.clone());
                                tile_image_offsets
                                    .insert((tileset_index, tile_id), tile_images.len() as u32);
                                tile_images.push(texture.clone());
                                dependencies.push(asset_path);
                            }
                        }

                        TilemapTexture::Vector(tile_images)
                    }
                    Some(img) => {
                        let tile_path = tmx_dir.join(&img.source.file_name().unwrap());
                        let asset_path = AssetPath::new(tile_path, None);
                        let texture: Handle<Image> = load_context.get_handle(asset_path.clone());
                        dependencies.push(asset_path);

                        TilemapTexture::Single(texture.clone())
                    }
                };

                tilemap_textures.insert(tileset_index, tilemap_texture);
            }

            let asset_map = TiledMap {
                map,
                tilemap_textures,
                tile_image_offsets,
            };

            log::info!("Loaded map: {}", load_context.path().display());

            let loaded_asset = LoadedAsset::new(asset_map);
            load_context.set_default_asset(loaded_asset.with_dependencies(dependencies));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["tmx"];
        EXTENSIONS
    }
}

fn tiled_pos_to_world_pos(map_size: &TilemapSize, grid_size: &TilemapGridSize, map_type: &TilemapType, z: f32, offset_x: f32, offset_y: f32, tile_map_height: u32, world_pos: Vec2) -> Vec3 {
    let tilemap_center_transform =
        get_tilemap_center_transform(
            map_size, grid_size, map_type, z,
        ) * Transform::from_xyz(offset_x, -offset_y, 0.0);

    // Get the tile pos as object position may not be what we need
    let mut tile_pos = TilePos::from_world_pos(
        &world_pos,
        &map_size,
        &grid_size,
        &TilemapType::Square,
    )
    .unwrap_or(TilePos::default());

    // We have different starting points for the tiles
    // tiled - upper left
    // bevy_ecs_tilemap - lower left
    tile_pos.y = (tile_map_height - 1) - tile_pos.y;

    let world_pos = tilemap_center_transform
        * tile_pos.center_in_world(&grid_size, &map_type).extend(0.0);

    world_pos
}

pub fn process_loaded_maps(
    mut commands: Commands,
    mut map_events: EventReader<AssetEvent<TiledMap>>,
    maps: Res<Assets<TiledMap>>,
    tile_storage_query: Query<(Entity, &TileStorage)>,
    mut map_query: Query<(&Handle<TiledMap>, &mut TiledLayersStorage)>,
    new_maps: Query<&Handle<TiledMap>, Added<Handle<TiledMap>>>,
    mut tileset_props: ResMut<TilesProperties>,
    mut signs_res: ResMut<SignsPool>,
    mut npc_res: ResMut<NpcPool>,
) {
    let mut changed_maps = Vec::<Handle<TiledMap>>::default();
    for event in map_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                log::info!("Map added!");
                changed_maps.push(handle.clone());
            }
            AssetEvent::Modified { handle } => {
                log::info!("Map changed!");
                changed_maps.push(handle.clone());
            }
            AssetEvent::Removed { handle } => {
                log::info!("Map removed!");
                // if mesh was modified and removed in the same update, ignore the modification
                // events are ordered so future modification events are ok
                changed_maps.retain(|changed_handle| changed_handle == handle);
            }
        }
    }

    // If we have new map entities add them to the changed_maps list.
    for new_map_handle in new_maps.iter() {
        changed_maps.push(new_map_handle.clone_weak());
    }

    for changed_map in changed_maps.iter() {
        for (map_handle, mut layer_storage) in map_query.iter_mut() {
            // only deal with currently changed map
            if map_handle != changed_map {
                continue;
            }

            let tile_map_opt = maps.get(map_handle);
            if tile_map_opt.is_none() {
                continue;
            }

            let tiled_map = tile_map_opt.unwrap();

            // TODO: Create a RemoveMap component..
            for layer_entity in layer_storage.storage.values() {
                if let Ok((_, layer_tile_storage)) = tile_storage_query.get(*layer_entity) {
                    for tile in layer_tile_storage.iter().flatten() {
                        commands.entity(*tile).despawn_recursive()
                    }
                }
                // commands.entity(*layer_entity).despawn_recursive();
            }

            // Process logic layers first. Their data is needed in order to insert
            // the proper properties for the actual layers' tiles.
            let mut logic_layers = std::collections::HashMap::new();
            for layer in tiled_map.map.layers() {
                let is_logic_layer = layer.name.starts_with("Logic");
                if !is_logic_layer {
                    continue;
                }

                let map_size = TilemapSize {
                    x: tiled_map.map.width,
                    y: tiled_map.map.height,
                };

                let parent;
                if let PropertyValue::StringValue(p) = &layer.properties["parent"] {
                    log::info!("Insert new logic layer: {}. Parent: {}", layer.name, p);
                    parent = p;
                } else {
                    // No parent layer
                    continue;
                }

                let tiled::LayerType::TileLayer(tile_layer) = layer.layer_type() else {
                    log::info!(
                        "Skipping layer {} because only tile layers are supported.",
                        layer.id()
                    );
                    continue;
                };
                let tiled::TileLayer::Finite(layer_data) = tile_layer else {
                    log::info!(
                        "Skipping layer {} because only finite layers are supported.",
                        layer.id()
                    );
                    continue;
                };

                // Do something with each tile from this logic layer.
                let mut v = Vec::new();
                v.resize(map_size.x as usize * map_size.y as usize, 0);
                for x in 0..map_size.x {
                    for y in 0..map_size.y {
                        let mut mapped_y = y;
                        if tiled_map.map.orientation == tiled::Orientation::Orthogonal {
                            mapped_y = (tiled_map.map.height - 1) - y;
                        }

                        let mapped_x = x as i32;
                        let mapped_y = mapped_y as i32;

                        let layer_tile = match layer_data.get_tile(mapped_x, mapped_y) {
                            Some(t) => t,
                            None => {
                                continue;
                            }
                        };

                        v[(mapped_y * map_size.x as i32 + mapped_x) as usize] = layer_tile.id();
                    }
                }

                logic_layers.insert(parent.clone(), v);
            }

            tileset_props.props.clear();
            tileset_props
                .props
                .resize(tiled_map.map.tilesets().len(), Vec::new());

            // The TilemapBundle requires that all tile images come exclusively from a single
            // tiled texture or from a Vec of independent per-tile images. Furthermore, all of
            // the per-tile images must be the same size. Since Tiled allows tiles of mixed
            // tilesets on each layer and allows differently-sized tile images in each tileset,
            // this means we need to load each combination of tileset and layer separately.
            for (tileset_index, tileset) in tiled_map.map.tilesets().iter().enumerate() {
                let Some(tilemap_texture) = tiled_map
                    .tilemap_textures
                    .get(&tileset_index) else {
                        log::warn!("Skipped creating layer with missing tilemap textures.");
                        continue;
                    };

                let tile_size = TilemapTileSize {
                    x: tileset.tile_width as f32,
                    y: tileset.tile_height as f32,
                };

                let tile_spacing = TilemapSpacing {
                    x: tileset.spacing as f32,
                    y: tileset.spacing as f32,
                };

                log::info!("Processing tileset: {}", tileset_index);

                tileset_props.props[tileset_index].resize(tileset.tiles().len(), HashMap::new());
                for (tile_id, tile) in tileset.tiles() {
                    tileset_props.props[tileset_index][tile_id as usize] = tile.properties.clone();
                }

                // Once materials have been created/added we need to then create the layers.
                for (layer_index, layer) in tiled_map.map.layers().enumerate() {
                    let is_logic_layer = layer.name.starts_with("Logic");
                    if is_logic_layer {
                        continue;
                    }

                    let map_size = TilemapSize {
                        x: tiled_map.map.width,
                        y: tiled_map.map.height,
                    };

                    let grid_size = TilemapGridSize {
                        x: tiled_map.map.tile_width as f32,
                        y: tiled_map.map.tile_height as f32,
                    };

                    let map_type = TilemapType::Square;

                    let offset_x = layer.offset_x;
                    let offset_y = layer.offset_y;

                    if let tiled::LayerType::ObjectLayer(obj_layer) = layer.layer_type() {
                        // Since object layers are not attached to specific tilesets
                        // as tile layers are, we process the obj layer with the first tileset
                        // that's why we skip it here if tileset_index != 0.
                        if tileset_index > 0 {
                            continue;
                        }

                        let objects: Vec<Object> = obj_layer.objects().collect();
                        log::info!(
                            "Processing object layer: {}. Object count: {}",
                            layer.name,
                            objects.len()
                        );

                        let mut signs = Vec::new();
                        let mut npcs = Vec::new();
                        for object in obj_layer.objects() {
                            if object.user_type == "npc" {
                                let id = match object.properties.get("id") {
                                    Some(tiled::PropertyValue::StringValue(id)) => id.clone(),
                                    _ => String::new(),
                                };

                                if id.is_empty() {
                                    continue;
                                }

                                let z = match object.properties.get("z") {
                                    Some(tiled::PropertyValue::IntValue(z)) => *z,
                                    _ => -1,
                                };

                                if z < 0 {
                                    continue;
                                }

                                let world_pos = tiled_pos_to_world_pos(&map_size, &grid_size, &map_type, z as f32, offset_x, offset_y, tiled_map.map.height, Vec2::new(object.x, object.y));

                                npcs.push(NpcData{name: id, pos: world_pos});
                            }

                            if object.user_type == "sign" {
                                let id = match object.properties.get("id") {
                                    Some(tiled::PropertyValue::IntValue(id)) => *id,
                                    _ => 0,
                                };

                                let z = match object.properties.get("z") {
                                    Some(tiled::PropertyValue::IntValue(z)) => *z as f32,
                                    _ => 0f32,
                                };

                                let world_pos = tiled_pos_to_world_pos(&map_size, &grid_size, &map_type, z, offset_x, offset_y, tiled_map.map.height, Vec2::new(object.x, object.y));

                                signs.push(SignData {
                                    x: world_pos.x,
                                    y: world_pos.y,
                                    id: id as u32,
                                });
                            }
                        }
                        signs_res.as_mut().signs = signs;
                        npc_res.as_mut().npcs = npcs;
                        continue;
                    }

                    if let Some(PropertyValue::IntValue(ts_index)) =
                        layer.properties.get("tileset_index")
                    {
                        if *ts_index as usize != tileset_index {
                            continue;
                        }
                    }

                    let mut z = -1;
                    if let Some(PropertyValue::IntValue(depth)) = layer.properties.get("z") {
                        z = *depth;
                    }

                    let tiled::LayerType::TileLayer(tile_layer) = layer.layer_type() else {
                        continue;
                    };

                    let tiled::TileLayer::Finite(layer_data) = tile_layer else {
                        log::info!(
                            "Skipping layer {} because only finite layers are supported.",
                            layer.id()
                        );
                        continue;
                    };

                    log::info!("Processing layer: {}", layer.name);

                    let mut tile_storage = TileStorage::empty(map_size);
                    let layer_entity = commands.spawn_empty().id();

                    let tilemap_center_transform =
                        get_tilemap_center_transform(&map_size, &grid_size, &map_type, z as f32)
                            * Transform::from_xyz(offset_x, -offset_y, 0.0);

                    let mut tilemap_empty = true;
                    for x in 0..map_size.x {
                        for y in 0..map_size.y {
                            let mut mapped_y = y;
                            if tiled_map.map.orientation == tiled::Orientation::Orthogonal {
                                mapped_y = (tiled_map.map.height - 1) - y;
                            }

                            let mapped_x = x as i32;
                            let mapped_y = mapped_y as i32;

                            let layer_tile = match layer_data.get_tile(mapped_x, mapped_y) {
                                Some(t) => t,
                                None => {
                                    continue;
                                }
                            };

                            assert!(layer_tile.tileset_index() == tileset_index);

                            let layer_tile_data = match layer_data.get_tile_data(mapped_x, mapped_y)
                            {
                                Some(d) => d,
                                None => {
                                    continue;
                                }
                            };

                            let texture_index = layer_tile.id();

                            let tile_pos = TilePos { x, y };
                            let tile_entity = commands
                                .spawn(TileBundle {
                                    position: tile_pos,
                                    tilemap_id: TilemapId(layer_entity),
                                    texture_index: TileTextureIndex(texture_index),
                                    flip: TileFlip {
                                        x: layer_tile_data.flip_h,
                                        y: layer_tile_data.flip_v,
                                        d: layer_tile_data.flip_d,
                                    },
                                    ..Default::default()
                                })
                                .id();

                            let collision = &layer_tile.get_tile().unwrap().collision;
                            if let Some(collisions) = collision {
                                for c in collisions.object_data() {
                                    let mut hw = 0.0;
                                    let mut hh = 0.0;
                                    let collider_entt = match &c.shape {
                                        ObjectShape::Rect { width, height } => {
                                            hw = width / 2.0;
                                            hh = height / 2.0;
                                            commands
                                                .spawn(Collider::cuboid(width / 2.0, height / 2.0))
                                                .id()
                                        }
                                        ObjectShape::Ellipse { width, height: _ } => {
                                            hw = width / 2.0;
                                            hh = hw;
                                            commands.spawn(Collider::ball(width / 2.0)).id()
                                        }
                                        _ => Entity::from_raw(0),
                                    };

                                    if collider_entt.index() != 0 {
                                        commands.entity(collider_entt).insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC);
                                    }

                                    let tile_world_pos = tilemap_center_transform
                                        * tile_pos
                                            .center_in_world(&grid_size, &map_type)
                                            .extend(0.0);

                                    // Offset since rapier places colliders such that
                                    // their center of mass lies on the given position
                                    // Tiled on the other hand uses offset from the upper-left
                                    // corner of the tile.
                                    commands.entity(collider_entt).insert(TransformBundle::from(
                                        Transform::from_translation(Vec3::new(
                                            c.x + (hw - tileset.tile_width as f32 / 2.0),
                                            tileset.tile_height as f32 / 2.0 - hh - c.y,
                                            0.0,
                                        )),
                                    ));

                                    commands
                                        .entity(tile_entity)
                                        .insert(RigidBody::Fixed)
                                        .insert(TransformBundle::from(Transform::from_translation(
                                            tile_world_pos,
                                        )))
                                        .add_child(collider_entt);
                                }
                            }

                            tile_storage.set(&tile_pos, tile_entity);
                            tilemap_empty = false;
                        }
                    }

                    // No need to spawn an empty tilemap
                    if tilemap_empty {
                        commands.entity(layer_entity).despawn();
                        continue;
                    }

                    let tilemap_bundle = TilemapBundle {
                        grid_size,
                        size: map_size,
                        storage: tile_storage,
                        texture: tilemap_texture.clone(),
                        tile_size,
                        spacing: tile_spacing,
                        transform: tilemap_center_transform,
                        map_type,
                        ..Default::default()
                    };
                    commands.entity(layer_entity).insert(tilemap_bundle);

                    layer_storage
                        .storage
                        .insert(layer_index as u32, layer_entity);
                }
            }
        }
    }
}
