use std::ops::{Range, RangeInclusive};

use assets::tileset_asset::TilesetAsset;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use camera::CameraLayer;
use common::{
    constants::TILE_SIZE,
    states::AppState,
    traits::{AddNamedObserver, Neighbors},
    types::ChunkCoordinates,
};

use crate::{
    chunk::{CHUNK_SIZE, ToChunkAndBlock, to_world_coordinates},
    chunk_visualisation::{
        ChunkVisualisation, ChunkVisualisationSettings,
        types::{TileType, Tilemap, Tilemaps, ToTiles},
    },
    messages::BlockUpdate,
    world_map::WorldMap,
};

pub fn plugin(app: &mut App) {
    app.register_type::<ChunkVisualisation>()
        .insert_resource(ChunkVisualisationSettings { visible_layers: 10 })
        .add_systems(
            Update,
            (update, request, delete).run_if(in_state(AppState::MainGame)),
        )
        .add_named_observer(on_insert, "on_chunk_vis_insert");
}

/// actually spawns chunk visualisations
///
/// Is called when a [`ChunkVisualisation`]` is inserted into an entity
pub(crate) fn on_insert(
    trigger: On<Insert, ChunkVisualisation>,
    chunk_visualisation_settings: Res<ChunkVisualisationSettings>,
    mut world_map: ResMut<WorldMap>,
    tileset: Res<TilesetAsset>,
    camera_layer: Single<&CameraLayer>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let target = trigger.entity;
    let chunk_visualisation = chunks.get(target).unwrap();
    world_map.ensure_surrounding_exist(chunk_visualisation.0);

    // First we  remove all children
    commands
        .entity(target)
        .despawn_related::<Children>()
        .insert(ChildOf(world_map.entity));

    // Setup hashmaps for tilemaps
    let mut tilemaps = Tilemaps::default();

    // we iterate over every x and y coordinate of the current chunk and go from the current camera layer downwards visible_layers + 1 layers
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            for z in 0..=chunk_visualisation_settings.visible_layers {
                let world_coordinates =
                    to_world_coordinates(chunk_visualisation.0, (x, y, 0))
                        // we begin at the camera layer. If we don't find a block, we step down a layer until we either find one
                        // or the opacity of the fog is too high to see
                        .with_z_offset(camera_layer.0 - z);
                if let Some(block) = world_map.get_block(world_coordinates) {
                    if block.to_tile(
                        x,
                        y,
                        z,
                        &mut tilemaps,
                        &world_coordinates,
                        world_map.as_ref(),
                        chunk_visualisation_settings.visible_layers as f32,
                    ) {
                        break;
                    }
                }
            }
        }
    }

    for (tile_type, map) in &tilemaps.0 {
        if !map.is_empty() {
            let tileset_image = match *tile_type {
                TileType::Full => &tileset.floor_tileset,
                TileType::Half => &tileset.soil_tileset,
                TileType::Animated => &tileset.water_tileset,
                TileType::Fog => &tileset.fog_tileset,
            };
            spawn_tile_map(&mut commands, tile_type, map, target, tileset_image);
        }
    }
}

fn spawn_tile_map(
    commands: &mut Commands,
    tile_type: &TileType,
    tile_map: &Tilemap,
    parent_chunk: Entity,
    tileset: &Handle<Image>,
) {
    let size = TilemapSize::from(tile_type.tilemap_size());
    let mut storage = TileStorage::empty(size);
    let tile_size = TilemapTileSize::from(tile_type.tile_size());
    let grid_size = tile_size.into();

    let tilemap_entity = commands.spawn(Name::from(tile_type.name().into())).id();

    commands
        .entity(tilemap_entity)
        .with_children(|parent| {
            for (tile_pos_type, tile_wrapper) in tile_map.iter() {
                tile_wrapper.spawn_bundle(
                    parent,
                    *tile_pos_type,
                    TilemapId(tilemap_entity),
                    &mut storage,
                );
            }
        })
        .insert(TilemapBundle {
            grid_size,
            map_type: TilemapType::Square,
            size,
            storage,
            texture: TilemapTexture::Single(tileset.clone()),
            tile_size,
            anchor: TilemapAnchor::BottomLeft,
            transform: Transform::from_xyz(0.0, 0.0, tile_type.z_offset()),
            ..default()
        })
        .insert(ChildOf(parent_chunk));
}

pub(crate) fn update(
    query: Query<(Entity, &ChunkVisualisation)>,
    mut message_reader: MessageReader<BlockUpdate>,
    mut commands: Commands,
) {
    for block_update in message_reader.read() {
        match block_update {
            BlockUpdate::Added => todo!(),
            BlockUpdate::Removed(world_coordinates) => {
                let (chunk_coordinates, block_coordinates) = world_coordinates.to_chunk_and_block();

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
            _ => (),
        }
    }
}

fn request(
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

fn delete(
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
