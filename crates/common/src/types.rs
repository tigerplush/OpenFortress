use bevy::{
    math::{IVec3, UVec3, Vec3},
    prelude::Component,
    reflect::Reflect,
};

use crate::traits::Neighbors;

/// These are essentially rounded world coordinates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct IWorldCoordinates(pub IVec3);

impl IWorldCoordinates {
    /// Manually overrides the z value of the coordinates.
    pub fn with_z_offset(mut self, z_offset: i32) -> IWorldCoordinates {
        self.0.z = z_offset;
        self
    }
}

impl Neighbors<IWorldCoordinates> for IWorldCoordinates {
    fn same_layer_neighbors(&self) -> Vec<(IWorldCoordinates, u32)> {
        self.0
            .same_layer_neighbors()
            .iter()
            .map(|(vec, cost)| (IWorldCoordinates(*vec), *cost))
            .collect()
    }

    fn all_neighbors(&self) -> Vec<(IWorldCoordinates, u32)> {
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

/// World coordinates that can be directly taken from a translation.
#[derive(Clone, Copy, Debug, PartialEq, Default, Reflect, Component)]
pub struct WorldCoordinates(pub Vec3);

impl WorldCoordinates {
    pub fn block(&self) -> IWorldCoordinates {
        IWorldCoordinates(self.0.round().as_ivec3())
    }
}

impl From<&IWorldCoordinates> for WorldCoordinates {
    fn from(wc: &IWorldCoordinates) -> WorldCoordinates {
        WorldCoordinates(wc.0.as_vec3())
    }
}
