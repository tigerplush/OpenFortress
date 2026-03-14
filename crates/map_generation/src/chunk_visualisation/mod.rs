use bevy::prelude::*;
use common::{constants::TILE_SIZE, types::ChunkCoordinates};

use crate::chunk::CHUNK_SIZE;

mod plugin;
pub use plugin::plugin;

/// Represents a visualised chunk. Most chunks only live in memory or on
/// storage. The visible chunks however are spawned in via ChunkVisualisations.
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
