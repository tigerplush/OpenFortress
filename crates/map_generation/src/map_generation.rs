use bevy::{platform_support::collections::HashMap, prelude::*};
use common::{
    states::AppState,
    traits::AddNamedObserver,
    types::{ChunkCoordinates, WorldCoordinates},
};
use noise::OpenSimplex;

use crate::{
    Chunk, ChunkVisualisation, ToChunkAndBlock, block_type::BlockType, chunk_visualisation,
    to_index,
};

pub fn plugin(app: &mut App) {
    app.register_type::<WorldMap>()
        .register_type::<ChunkVisualisation>()
        .add_systems(OnEnter(AppState::MainGame), spawn_world)
        .add_systems(
            Update,
            (chunk_visualisation::request, chunk_visualisation::delete)
                .run_if(in_state(AppState::MainGame)),
        )
        .add_named_observer(chunk_visualisation::on_insert, "on_chunk_vis_insert")
        .add_named_observer(
            chunk_visualisation::on_chunk_visualisation_event,
            "on_chunk_vis_event",
        );
}

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
    fn new(entity: Entity) -> Self {
        WorldMap {
            chunks: HashMap::default(),
            noise: OpenSimplex::new(0),
            entity,
            block_states: HashMap::default(),
        }
    }

    pub(crate) fn get_chunk(&mut self, coordinates: ChunkCoordinates) -> &Chunk {
        self.get_or_insert_chunk_mut(coordinates)
    }

    /// Returns a chunk for a given coordinate. Will create a new one, if none has been created thus far.
    fn get_or_insert_chunk_mut(&mut self, coordinates: ChunkCoordinates) -> &mut Chunk {
        self.chunks
            .entry(coordinates.0)
            .or_insert(Chunk::new(coordinates, self.noise))
    }

    pub fn get_block(&self, coordinates: WorldCoordinates) -> Option<BlockType> {
        let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();
        let index = to_index(block_coordinates);
        self.chunks
            .get(&chunk_coordinates.0)
            .and_then(|chunk| match chunk.blocks[index] {
                BlockType::None => None,
                _ => Some(chunk.blocks[index]),
            })
    }

    pub fn solidness(&self, coordinates: WorldCoordinates) -> bool {
        let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();
        let index = to_index(block_coordinates);
        self.chunks
            .get(&chunk_coordinates.0)
            .is_none_or(|chunk| chunk.blocks[index].is_solid())
    }

    /// Adds damage to a block. Returns true, if the block is destroyed, false otherwise.
    pub fn damage_block(&mut self, coordinates: WorldCoordinates, damage: f32) -> bool {
        let remaining_health = {
            *self
                .block_states
                .entry(coordinates.0)
                .and_modify(|block| *block -= damage)
                .or_insert(1.0)
        };
        if remaining_health < 0.0 {
            let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();
            self.get_or_insert_chunk_mut(chunk_coordinates)
                .remove_block(block_coordinates);
        }
        remaining_health < 0.0
    }
}

fn spawn_world(mut commands: Commands) {
    let entity = commands
        .spawn((
            Name::new("World Map"),
            Transform::default(),
            Visibility::Inherited,
        ))
        .id();
    commands.insert_resource(WorldMap::new(entity));
}
