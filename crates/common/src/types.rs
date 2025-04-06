use bevy::{
    math::{IVec3, UVec3},
    reflect::Reflect,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct WorldCoordinates(pub IVec3);

/// Coordinates of a chunk within the world
#[derive(Clone, Copy, PartialEq, Reflect)]
pub struct ChunkCoordinates(pub IVec3);
/// Coordinates of a block within a chunk
pub struct BlockCoordinates(pub UVec3);
