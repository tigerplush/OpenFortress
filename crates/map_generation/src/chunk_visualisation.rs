use assets::tileset_asset::TilesetAsset;
use bevy::{color::palettes::css::WHITE, platform_support::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::{
    TilemapBundle,
    anchor::TilemapAnchor,
    map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType},
    tiles::{AnimatedTile, TileBundle, TileColor, TilePos, TileStorage, TileTextureIndex},
};
use camera::CameraLayer;
use common::{
    constants::TILE_SIZE,
    traits::Neighbors,
    types::{ChunkCoordinates, WorldCoordinates},
};
use std::ops::{Range, RangeInclusive};

use crate::{
    ToChunkAndBlock,
    block_type::BlockType,
    chunk::{CHUNK_SIZE, to_world_coordinates},
    map_generation::WorldMap,
};

/// actually spawns chunk visualisations
///
/// Is called when a [`ChunkVisualisation`]` is inserted into an entity
pub(crate) fn on_insert(
    trigger: Trigger<OnInsert, ChunkVisualisation>,
    mut world_map: ResMut<WorldMap>,
    tileset: Res<TilesetAsset>,
    camera_layer: Single<&CameraLayer>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let target = trigger.target();
    let chunk_visualisation = chunks.get(target).unwrap();
    world_map.ensure_surrounding_exist(chunk_visualisation.0);

    // First we  remove all children
    commands
        .entity(target)
        .despawn_related::<Children>()
        .insert(ChildOf(world_map.entity));

    // Setup hashmaps for tilemaps
    let mut fog_tiles = HashMap::new();
    let mut water_tiles = HashMap::new();
    let mut full_tiles = HashMap::new();
    let mut half_tiles = HashMap::new();

    // we iterate over every x and y coordinate of the current chunk and go from the current camera layer downwards 11 layers
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            for z in 0..11 {
                let current_world_coordinates =
                    to_world_coordinates(chunk_visualisation.0, (x, y, 0))
                        // we begin at the camera layer. If we don't find a block, we step down a layer until we either find one
                        // or the opacity of the fog is too high to see
                        .with_z_offset(camera_layer.0 - z);
                if let Some(block) = world_map.get_block(current_world_coordinates) {
                    match block {
                        BlockType::Solid(_) => {
                            if z == 0 {
                                // if the current block is solid and at the current camera z layer, we add it to the half tiles
                                // add half tiles
                                let mut flags = 0;
                                for (index, (neighbor, _)) in current_world_coordinates
                                    .same_layer_neighbors()
                                    .iter()
                                    .enumerate()
                                {
                                    // fetch the block
                                    // check if its solid
                                    let solid: u8 = world_map.solidness(*neighbor).into();
                                    // add its state to the flag
                                    flags |= solid << index;
                                }
                                half_tiles.insert((x, y), (block, flags));
                            } else {
                                // if the current block is solid and not at the current camera z layer, we add it to the full floor tiles
                                full_tiles.insert(TilePos::new(x, y), TileWrapper::Floor(block));
                            }
                        }
                        BlockType::Liquid => {
                            // if the current block is liquid, we add it to the animated water tiles
                            water_tiles.insert(TilePos::new(x, y), TileWrapper::Water);
                        }
                        _ => (),
                    }
                    break;
                } else if z != 0 {
                    // for every z layer from current camera layer -1 to current camera layer -10, we add fog with the respecting opacity
                    fog_tiles.insert(TilePos::new(x, y), TileWrapper::Fog(z as f32 / 10.0));
                }
            }
        }
    }

    // if the half tiles aren't empty, spawn the tilemap for the half tiles
    if !half_tiles.is_empty() {
        let map_size = TilemapSize::from(CHUNK_SIZE.truncate() * 2);
        let mut tile_storage = TileStorage::empty(map_size);

        let tile_size = TilemapTileSize::from(TILE_SIZE / 2.0);
        let grid_size = tile_size.into();
        let tilemap_entity = commands.spawn(Name::new("Tilemap")).id();
        commands
            .entity(tilemap_entity)
            .with_children(|parent| {
                for ((x, y), (block_type, flags)) in half_tiles.iter() {
                    block_type.spawn_half_tile(
                        parent,
                        *x,
                        *y,
                        tilemap_entity,
                        &mut tile_storage,
                        *flags,
                    );
                }
            })
            .insert(TilemapBundle {
                grid_size,
                map_type: TilemapType::Square,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(tileset.soil_tileset.clone_weak()),
                tile_size,
                anchor: TilemapAnchor::BottomLeft,
                ..default()
            })
            .insert(ChildOf(target));
    }

    // if the full (floor tiles) aren't empty, spawn the tilemap for floors
    if !full_tiles.is_empty() {
        spawn_tile_map(
            &mut commands,
            &full_tiles,
            target,
            &tileset.floor_tileset,
            -0.5,
        );
    }

    // if the liquid tiles aren't empty, spawn the tilemap for animated liquids
    if !water_tiles.is_empty() {
        spawn_tile_map(
            &mut commands,
            &water_tiles,
            target,
            &tileset.water_tileset,
            -0.5,
        );
    }

    // if the fog tiles aren't empty, spawn the tilemap for fog
    if !fog_tiles.is_empty() {
        spawn_tile_map(&mut commands, &fog_tiles, target, &tileset.fog_tileset, 1.0);
    }
}

/// Wrapper to wrap the contents of a full-tile tilemap
enum TileWrapper {
    /// Wrapper for a fog type, carries the opacity in percent (ranging 0.0 to 1.0)
    Fog(f32),
    /// Wrapper for a water type
    Water,
    /// Wrapper for a block type
    Floor(BlockType),
}

/// spawns a full-tile tilemap
fn spawn_tile_map(
    commands: &mut Commands,
    tiles: &HashMap<TilePos, TileWrapper>,
    target: Entity,
    tileset: &Handle<Image>,
    z_offset: f32,
) {
    let size = TilemapSize::from(CHUNK_SIZE.truncate());
    let mut storage = TileStorage::empty(size);
    let tile_size = TilemapTileSize::from(TILE_SIZE);
    let grid_size = tile_size.into();
    let tilemap_entity = commands.spawn(Name::new("Fog Tilemap")).id();
    commands
        .entity(tilemap_entity)
        .with_children(|parent| {
            for (tile_pos, opacity) in tiles.iter() {
                let tile_entity = match opacity {
                    TileWrapper::Fog(opacity) => parent
                        .spawn(TileBundle {
                            position: *tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            color: TileColor(WHITE.with_alpha(*opacity).into()),
                            ..default()
                        })
                        .id(),
                    TileWrapper::Water => parent
                        .spawn((
                            TileBundle {
                                position: *tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                texture_index: TileTextureIndex(0),
                                ..default()
                            },
                            AnimatedTile {
                                start: 0,
                                end: 8,
                                speed: 0.1,
                            },
                        ))
                        .id(),
                    TileWrapper::Floor(block) => parent
                        .spawn(TileBundle {
                            position: *tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            ..block.floor_tile()
                        })
                        .id(),
                };
                storage.set(tile_pos, tile_entity);
            }
        })
        .insert(TilemapBundle {
            grid_size,
            map_type: TilemapType::Square,
            size,
            storage,
            texture: TilemapTexture::Single(tileset.clone_weak()),
            tile_size,
            anchor: TilemapAnchor::BottomLeft,
            transform: Transform::from_xyz(0.0, 0.0, z_offset),
            ..default()
        })
        .insert(ChildOf(target));
}

pub(crate) fn on_chunk_visualisation_event(
    trigger: Trigger<ChunkVisualisationEvent>,
    query: Query<(Entity, &ChunkVisualisation)>,
    mut commands: Commands,
) {
    let ChunkVisualisationEvent::SetDirty(coordinates) = trigger.event();
    let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();

    let mut all = vec![chunk_coordinates];
    if block_coordinates.0.x == 0
        || block_coordinates.0.y == 0
        || block_coordinates.0.x == CHUNK_SIZE.x - 1
        || block_coordinates.0.y == CHUNK_SIZE.y - 1
    {
        let neighbors: Vec<ChunkCoordinates> = chunk_coordinates
            .0
            .same_layer_neighbors()
            .iter()
            .map(|(coordinate, _)| ChunkCoordinates(*coordinate))
            .collect();
        all.extend(neighbors);
    }
    for coordinates in all {
        if let Some((entity, _)) = query
            .iter()
            .find(|(_, chunk_vis)| chunk_vis.0 == coordinates)
        {
            commands
                .entity(entity)
                .insert(ChunkVisualisation(coordinates));
        }
    }
}

#[derive(Event)]
pub enum ChunkVisualisationEvent {
    SetDirty(WorldCoordinates),
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ChunkVisualisation(ChunkCoordinates);

impl ChunkVisualisation {
    fn bundle(coordinates: ChunkCoordinates) -> impl Bundle {
        (
            Name::new(format!("Chunk {}", coordinates.0)),
            ChunkVisualisation(coordinates),
            Transform::from_xyz(
                coordinates.0.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
                coordinates.0.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
                0.0,
            ),
            Visibility::Inherited,
        )
    }
}

pub(crate) fn request(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let Some((x_range, y_range, z_range)) =
        calculate_visible_chunk_ranges_from_single(camera_transform)
    else {
        return;
    };

    let mut requested_chunks = vec![];
    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                requested_chunks.push(ChunkCoordinates(IVec3::new(x, y, z)));
            }
        }
    }
    for coordinates in requested_chunks {
        // are the chunks already there?
        if !chunks.iter().any(|chunk| chunk.0 == coordinates) {
            // if not, spawn them
            commands.spawn(ChunkVisualisation::bundle(coordinates));
        }
    }
}

pub(crate) fn delete(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
    chunks: Query<(Entity, &ChunkVisualisation)>,
    mut commands: Commands,
) {
    let Some((x_range, y_range, z_range)) =
        calculate_visible_chunk_ranges_from_single(camera_transform)
    else {
        return;
    };
    for (entity, chunk) in &chunks {
        let coordinates = chunk.0;
        if !x_range.contains(&coordinates.0.x)
            || !y_range.contains(&coordinates.0.y)
            || !z_range.contains(&coordinates.0.z)
        {
            debug!("despawning chunk {}", entity);
            commands.entity(entity).despawn();
        }
    }
}

fn calculate_visible_chunk_ranges_from_single(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
) -> Option<(Range<i32>, Range<i32>, RangeInclusive<i32>)> {
    let (transform, layer, projection) = camera_transform.into_inner();
    let Projection::Orthographic(values) = projection else {
        return None;
    };
    Some(calculate_visible_chunk_ranges(transform, layer, values))
}

/// Calculates which chunks are currently visible
fn calculate_visible_chunk_ranges(
    transform: &Transform,
    layer: &CameraLayer,
    projection: &OrthographicProjection,
) -> (Range<i32>, Range<i32>, RangeInclusive<i32>) {
    let camera_x = transform.translation.x;
    let camera_y = transform.translation.y;

    let chunk_size_x = CHUNK_SIZE.x as f32 * TILE_SIZE.x;
    let chunk_size_y = CHUNK_SIZE.y as f32 * TILE_SIZE.y;
    let min_x = camera_x + projection.area.min.x;
    let max_x = camera_x + projection.area.max.x;
    let min_y = camera_y + projection.area.min.y;
    let max_y = camera_y + projection.area.max.y;

    let min_chunk_x = (min_x / chunk_size_x).floor() as i32;
    let max_chunk_x = (max_x / chunk_size_x).ceil() as i32;
    let min_chunk_y = (min_y / chunk_size_y).floor() as i32;
    let max_chunk_y = (max_y / chunk_size_y).ceil() as i32;

    (
        min_chunk_x..max_chunk_x,
        min_chunk_y..max_chunk_y,
        // (layer.0 - 1)..=(layer.0 + 1),
        layer.0..=layer.0,
    )
}
