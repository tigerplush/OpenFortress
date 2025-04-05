use assets::tileset_asset::{BlockType, TilesetAsset};
use bevy::prelude::*;
use camera::CameraLayer;
use common::{constants::TILE_SIZE, traits::AsVec2};
use noise::{NoiseFn, OpenSimplex};
use std::ops::{Range, RangeInclusive};

use crate::WorldMap;

pub(crate) fn on_add_visualisation(
    trigger: Trigger<OnInsert, ChunkVisualisation>,
    mut world_map: ResMut<WorldMap>,
    tileset: Res<TilesetAsset>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let chunk_visualisation = chunks.get(trigger.target()).unwrap();
    let chunk = world_map.get_or_insert_chunk_mut(chunk_visualisation.0);
    commands
        .entity(trigger.target())
        .despawn_related::<Children>()
        .with_children(|parent| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    for z in (0..CHUNK_SIZE.z).rev() {
                        let index = to_index((x, y, z));
                        if chunk.blocks[index] != BlockType::None {
                            parent.spawn((
                                Name::new(format!("Block [{}, {}, {}]", x, y, z)),
                                Sprite {
                                    image: tileset.image.clone_weak(),
                                    texture_atlas: Some(TextureAtlas {
                                        layout: tileset.layout_handle.clone_weak(),
                                        index: chunk.blocks[index].into(),
                                    }),
                                    ..default()
                                },
                                Transform::from_translation(
                                    ((x, y).as_vec2() * TILE_SIZE).extend(-1.0),
                                ),
                            ));
                            break;
                        }
                    }
                }
            }
            chunk.is_dirty = false;
        })
        .insert(ChildOf {
            parent: world_map.entity,
        });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct ChunkVisualisation(IVec3);

impl ChunkVisualisation {
    fn bundle(coordinates: IVec3) -> impl Bundle {
        (
            Name::new(format!("Chunk {}", coordinates)),
            ChunkVisualisation(coordinates),
            Transform::from_xyz(
                coordinates.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
                coordinates.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
                coordinates.z as f32,
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
                requested_chunks.push(IVec3::new(x, y, z));
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

pub(crate) fn update(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
    world_map: Res<WorldMap>,
    chunks: Query<(Entity, &ChunkVisualisation)>,
    mut commands: Commands,
) {
    let Some((x_range, y_range, z_range)) =
        calculate_visible_chunk_ranges_from_single(camera_transform)
    else {
        return;
    };

    for (entity, chunk_vis) in chunks.iter().filter(|(_, chunk)| {
        x_range.contains(&chunk.0.x) && y_range.contains(&chunk.0.y) && z_range.contains(&chunk.0.z)
    }) {
        if let Some(chunk) = world_map.get_chunk(&chunk_vis.0) {
            if chunk.is_dirty {
                commands
                    .entity(entity)
                    .insert(ChunkVisualisation(chunk_vis.0));
            }
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
        if !x_range.contains(&coordinates.x)
            || !y_range.contains(&coordinates.y)
            || !z_range.contains(&coordinates.z)
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

const CHUNK_SIZE: UVec3 = UVec3::new(16, 16, 1);

#[derive(Reflect)]
pub(crate) struct Chunk {
    coordinates: IVec3,
    pub(crate) blocks: [BlockType; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize],
    is_dirty: bool,
}

impl Chunk {
    pub(crate) fn new(coordinates: IVec3, noise: OpenSimplex) -> Self {
        let mut blocks = [BlockType::None; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize];
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let world_x = coordinates.x as f32 + (x as f32 / CHUNK_SIZE.x as f32);
                let world_y = coordinates.y as f32 + (y as f32 / CHUNK_SIZE.y as f32);
                let threshold = noise
                    .get([world_x as f64, world_y as f64])
                    .remap(-1.0, 1.0, -10984.0, 8848.0)
                    .round() as i32;
                for z in 0..CHUNK_SIZE.z {
                    let height = coordinates.z * CHUNK_SIZE.z as i32 + z as i32;
                    let tile_type = if height == threshold && threshold > 0 {
                        BlockType::BrightGrass
                    } else if height < threshold {
                        BlockType::Dirt
                    } else if height > threshold && height < 0 {
                        BlockType::Water
                    } else {
                        BlockType::None
                    };
                    blocks[to_index((x, y, z))] = tile_type;
                }
            }
        }
        Chunk {
            coordinates,
            blocks,
            is_dirty: true,
        }
    }

    pub(crate) fn remove_block(&mut self, block_coordinates: UVec3) {
        let index = to_index(block_coordinates.into());
        self.blocks[index] = BlockType::None;
        self.is_dirty = true;
    }
}

pub(crate) trait ToChunkAndBlock {
    fn to_chunk_and_block(&self) -> (IVec3, UVec3);
}

impl ToChunkAndBlock for IVec3 {
    fn to_chunk_and_block(&self) -> (IVec3, UVec3) {
        (
            self.div_euclid(CHUNK_SIZE.as_ivec3()),
            self.rem_euclid(CHUNK_SIZE.as_ivec3()).as_uvec3(),
        )
    }
}

/// returns the index of a tile in it's block array by coordinates
pub(crate) fn to_index(block_coordinates: (u32, u32, u32)) -> usize {
    (block_coordinates.0 * CHUNK_SIZE.y * CHUNK_SIZE.z
        + block_coordinates.1 * CHUNK_SIZE.z
        + block_coordinates.2) as usize
}

#[test]
fn test_to_index() {
    let mut index = 0;
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            for z in 0..CHUNK_SIZE.z {
                assert_eq!(to_index((x, y, z)), index, "x: {}, y: {}, z: {}", x, y, z);
                index += 1;
            }
        }
    }
}
