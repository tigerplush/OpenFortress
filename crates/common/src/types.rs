use bevy::{
    math::{IVec3, UVec3},
    reflect::Reflect,
};

use crate::traits::Neighbors;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct WorldCoordinates(pub IVec3);

impl WorldCoordinates {
    /// Manually overrides the z value of the coordinates.
    pub fn with_z_offset(mut self, z_offset: i32) -> WorldCoordinates {
        self.0.z = z_offset;
        self
    }
}

impl Neighbors<WorldCoordinates> for WorldCoordinates {
    fn same_layer_neighbors(&self) -> Vec<(WorldCoordinates, u32)> {
        self.0
            .same_layer_neighbors()
            .iter()
            .map(|(vec, cost)| (WorldCoordinates(*vec), *cost))
            .collect()
    }

    fn all_neighbors(&self) -> Vec<(WorldCoordinates, u32)> {
        todo!("not implemented")
    }
}

/// Coordinates of a chunk within the world
#[derive(Clone, Copy, PartialEq, Reflect)]
pub struct ChunkCoordinates(pub IVec3);
/// Coordinates of a block within a chunk
pub struct BlockCoordinates(pub UVec3);

impl From<(u32, u32, u32)> for BlockCoordinates {
    fn from(value: (u32, u32, u32)) -> Self {
        BlockCoordinates(UVec3::new(value.0, value.1, value.2))
    }
}
