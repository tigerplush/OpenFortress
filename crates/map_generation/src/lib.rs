use assets::tileset_asset::BlockType;
use bevy::{platform_support::collections::HashMap, prelude::*};
use chunk::{Chunk, ChunkVisualisation, ToChunkAndBlock, to_index};
use common::states::AppState;
use noise::OpenSimplex;

mod chunk;

pub fn plugin(app: &mut App) {
    app.register_type::<WorldMap>()
        .register_type::<ChunkVisualisation>()
        .add_systems(OnEnter(AppState::MainGame), spawn_world)
        .add_systems(
            Update,
            (chunk::request, chunk::update, chunk::delete).run_if(in_state(AppState::MainGame)),
        )
        .add_observer(chunk::on_add_visualisation);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WorldMap {
    chunks: HashMap<IVec3, Chunk>,
    #[reflect(ignore)]
    noise: OpenSimplex,
    entity: Entity,
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

    fn get_chunk(&self, coordinates: &IVec3) -> Option<&Chunk> {
        self.chunks.get(coordinates)
    }

    /// Returns a chunk for a given coordinate. Will create a new one, if none has been created thus far.
    fn get_or_insert_chunk_mut(&mut self, coordinates: IVec3) -> &mut Chunk {
        self.chunks
            .entry(coordinates)
            .or_insert(Chunk::new(coordinates, self.noise))
    }

    pub fn get_block(&self, coordinates: IVec3) -> Option<BlockType> {
        let (chunk_coordinates, block_coordinates) = coordinates.to_chunk_and_block();
        let index = to_index(block_coordinates.into());
        self.chunks
            .get(&chunk_coordinates)
            .and_then(|chunk| match chunk.blocks[index] {
                BlockType::None => None,
                _ => Some(chunk.blocks[index]),
            })
    }

    /// Adds damage to a block. Returns true, if the block is destroyed, false otherwise.
    pub fn damage_block(&mut self, coordinates: IVec3, damage: f32) -> bool {
        let remaining_health = {
            *self
                .block_states
                .entry(coordinates)
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
