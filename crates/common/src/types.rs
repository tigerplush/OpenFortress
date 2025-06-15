use bevy::{
    math::{IVec3, UVec3},
    reflect::Reflect,
};

use crate::traits::Neighbors;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct BlockCoordinates(pub IVec3);

impl BlockCoordinates {
    /// Manually overrides the z value of the coordinates.
    pub fn with_z_offset(mut self, z_offset: i32) -> BlockCoordinates {
        self.0.z = z_offset;
        self
    }
}

impl Neighbors<BlockCoordinates> for BlockCoordinates {
    fn same_layer_neighbors(&self) -> Vec<(BlockCoordinates, u32)> {
        self.0
            .same_layer_neighbors()
            .iter()
            .map(|(vec, cost)| (BlockCoordinates(*vec), *cost))
            .collect()
    }

    fn all_neighbors(&self) -> Vec<(BlockCoordinates, u32)> {
        todo!("not implemented")
    }
}

/// Coordinates of a chunk within the world
#[derive(Clone, Copy, PartialEq, Reflect)]
pub struct ChunkCoordinates(pub IVec3);
/// Coordinates of a block within a chunk
pub struct ChunkBlockCoordinates(pub UVec3);

impl From<(u32, u32, u32)> for ChunkBlockCoordinates {
    fn from(value: (u32, u32, u32)) -> Self {
        ChunkBlockCoordinates(UVec3::new(value.0, value.1, value.2))
    }
}
