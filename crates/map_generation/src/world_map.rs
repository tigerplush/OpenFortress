use std::collections::HashMap;

use bevy::prelude::*;
use common::{
    traits::Neighbors,
    types::{ChunkCoordinates, IWorldCoordinates},
};
use noise::OpenSimplex;

use crate::{
    block_type::BlockType,
    chunk::{Chunk, ToChunkAndBlock, to_index},
};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WorldMap {
    chunks: HashMap<IVec3, Chunk>,
    #[reflect(ignore)]
    noise: OpenSimplex,
    pub(crate) entity: Entity,
    block_states: HashMap<IVec3, f32>,
}

impl WorldMap {
    pub(crate) fn new(entity: Entity, seed: u32) -> Self {
        WorldMap {
            chunks: HashMap::default(),
            noise: OpenSimplex::new(seed),
            entity,
            block_states: HashMap::default(),
        }
    }

    /// Checks every surrounding chunk. If it doesn't exist, it will be created.
    pub(crate) fn ensure_surrounding_exist(&mut self, coordinates: ChunkCoordinates) {
        self.get_or_insert_chunk_mut(coordinates);
        for (neighbor, _) in coordinates.0.all_neighbors() {
            self.get_or_insert_chunk_mut(ChunkCoordinates(neighbor));
        }
    }

    /// Returns a chunk for a given coordinate. Will create a new one, if none has been created thus far.
    fn get_or_insert_chunk_mut(&mut self, coordinates: ChunkCoordinates) -> &mut Chunk {
        self.chunks
            .entry(coordinates.0)
            .or_insert(Chunk::new(coordinates, self.noise))
    }

    /// Returns an option of type BlockType, if the corresponding chunk has been
    /// found. Returns None when the chunk is not loaded.
    pub fn get_raw_block(&self, coordinates: IWorldCoordinates) -> Option<BlockType> {
        let (chunk_coordinate, block_coordinates) = coordinates.to_chunk_and_block();
        let index = to_index(block_coordinates);
        self.chunks
            .get(&chunk_coordinate.0)
            .map(|chunk| chunk.blocks[index])
    }

    pub fn solidness(&self, coordinates: IWorldCoordinates) -> bool {
        let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();
        let index = to_index(block_coordinates);
        self.chunks
            .get(&chunk_coordinates.0)
            .is_none_or(|chunk| chunk.blocks[index].is_solid())
    }

    /// Adds damage to a block. Returns true, if the block is destroyed, false otherwise.
    pub fn damage_block(&mut self, coordinates: IWorldCoordinates, damage: f32) -> bool {
        let remaining_health = {
            *self
                .block_states
                .entry(coordinates.0)
                .and_modify(|block| *block -= damage)
                .or_insert(1.0 - damage)
        };
        if remaining_health <= 0.0 {
            let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();
            self.get_or_insert_chunk_mut(chunk_coordinates)
                .remove_block(block_coordinates);
        }
        remaining_health < 0.0
    }
}
