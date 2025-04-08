use bevy::prelude::*;
use common::types::{BlockCoordinates, ChunkCoordinates, WorldCoordinates};
use noise::{NoiseFn, OpenSimplex};

use crate::block_type::BlockType;

pub(crate) const CHUNK_SIZE: UVec3 = UVec3::new(16, 16, 1);

#[derive(Reflect)]
pub(crate) struct Chunk {
    pub(crate) coordinates: ChunkCoordinates,
    pub(crate) blocks: [BlockType; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize],
}

impl Chunk {
    pub(crate) fn new(coordinates: ChunkCoordinates, noise: OpenSimplex) -> Self {
        let mut blocks = [BlockType::None; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize];
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let world_x = coordinates.0.x as f32 + (x as f32 / CHUNK_SIZE.x as f32);
                let world_y = coordinates.0.y as f32 + (y as f32 / CHUNK_SIZE.y as f32);
                let threshold = noise
                    .get([world_x as f64, world_y as f64])
                    .remap(-1.0, 1.0, -10984.0, 8848.0)
                    .round() as i32;
                for z in 0..CHUNK_SIZE.z {
                    let height = coordinates.0.z * CHUNK_SIZE.z as i32 + z as i32;
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
        }
    }

    pub(crate) fn remove_block(&mut self, block_coordinates: BlockCoordinates) {
        let index = to_index(block_coordinates);
        self.blocks[index] = BlockType::None;
    }
}

pub(crate) trait ToChunkAndBlock {
    fn to_chunk_and_block(&self) -> (ChunkCoordinates, BlockCoordinates);
}

impl ToChunkAndBlock for WorldCoordinates {
    fn to_chunk_and_block(&self) -> (ChunkCoordinates, BlockCoordinates) {
        (
            ChunkCoordinates(self.0.div_euclid(CHUNK_SIZE.as_ivec3())),
            BlockCoordinates(self.0.rem_euclid(CHUNK_SIZE.as_ivec3()).as_uvec3()),
        )
    }
}

/// returns the index of a tile in it's block array by coordinates
pub(crate) fn to_index(coordinates: impl Into<BlockCoordinates>) -> usize {
    let block_coordinates: BlockCoordinates = coordinates.into();
    (block_coordinates.0.x * CHUNK_SIZE.y * CHUNK_SIZE.z
        + block_coordinates.0.y * CHUNK_SIZE.z
        + block_coordinates.0.z) as usize
}

pub(crate) fn to_world_coordinates(
    chunk_coordinates: ChunkCoordinates,
    block_coordinates: impl Into<BlockCoordinates>,
) -> WorldCoordinates {
    let block_coordinates: BlockCoordinates = block_coordinates.into();
    let x = chunk_coordinates.0.x * CHUNK_SIZE.x as i32 + block_coordinates.0.x as i32;
    let y = chunk_coordinates.0.y * CHUNK_SIZE.y as i32 + block_coordinates.0.y as i32;
    let z = chunk_coordinates.0.z * CHUNK_SIZE.z as i32 + block_coordinates.0.z as i32;
    WorldCoordinates(IVec3::new(x, y, z))
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

#[test]
fn test_to_world() {
    let chunk_coordinates = ChunkCoordinates(IVec3::ZERO);
    assert_eq!(
        to_world_coordinates(chunk_coordinates, (1, 2, 3)),
        WorldCoordinates(IVec3::new(1, 2, 3))
    )
}
