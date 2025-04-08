use assets::tileset_asset::TilesetAsset;
use bevy::prelude::*;
use camera::CameraLayer;
use common::{
    constants::TILE_SIZE,
    traits::{AsVec2, Neighbors},
    types::{ChunkCoordinates, WorldCoordinates},
};
use std::ops::{Range, RangeInclusive};

use crate::{
    ToChunkAndBlock,
    chunk::{CHUNK_SIZE, to_world_coordinates},
    map_generation::WorldMap,
};

pub(crate) fn on_insert(
    trigger: Trigger<OnInsert, ChunkVisualisation>,
    mut world_map: ResMut<WorldMap>,
    tileset: Res<TilesetAsset>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let chunk_visualisation = chunks.get(trigger.target()).unwrap();
    world_map.get_chunk(chunk_visualisation.0);
    commands
        .entity(trigger.target())
        .despawn_related::<Children>()
        .with_children(|parent| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    for z in (0..CHUNK_SIZE.z).rev() {
                        // fetch world coordinates for current block
                        let current_world_coordinates =
                            to_world_coordinates(chunk_visualisation.0, (x, y, z));
                        if let Some(current_block) = world_map.get_block(current_world_coordinates)
                        {
                            // init flags
                            let mut flags = 0;
                            //iterate over every neighbor
                            for (index, (neighbor, _)) in
                                current_world_coordinates.neighbors().iter().enumerate()
                            {
                                // fetch the block
                                // check if its solid
                                let solid: u8 = world_map.solidness(*neighbor).into();
                                // add its state to the flag
                                flags |= solid << index;
                            }
                            parent.spawn((
                                Name::new(format!("Block [{}, {}, {}]", x, y, z)),
                                current_block.sprite(&tileset, flags),
                                Transform::from_translation(
                                    ((x, y).as_vec2() * TILE_SIZE).extend(-1.0),
                                ),
                            ));
                        }
                    }
                }
            }
        })
        .insert(ChildOf {
            parent: world_map.entity,
        });
}

pub(crate) fn on_chunk_visualisation_event(
    trigger: Trigger<ChunkVisualisationEvent>,
    query: Query<(Entity, &ChunkVisualisation)>,
    mut commands: Commands,
) {
    let ChunkVisualisationEvent::SetDirty(coordinates) = trigger.event();
    let (chunk_coordinates, _) = coordinates.to_chunk_and_block();
    if let Some((entity, _)) = query
        .iter()
        .find(|(_, chunk_vis)| chunk_vis.0 == chunk_coordinates)
    {
        commands
            .entity(entity)
            .insert(ChunkVisualisation(chunk_coordinates));
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
                coordinates.0.z as f32,
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
