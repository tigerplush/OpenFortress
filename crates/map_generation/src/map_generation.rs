use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_tilemap::TilemapPlugin;
use common::{
    constants::TILE_SIZE,
    states::AppState,
    traits::{AddNamedObserver, Neighbors},
    types::{ChunkCoordinates, WorldCoordinates},
};
use noise::OpenSimplex;

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct WorldGenerationSettings {
    pub seed: u32,
}

use crate::{
    Chunk, ChunkVisualisation, ToChunkAndBlock, block_type::BlockType, chunk_visualisation,
    to_index,
};

pub fn plugin(app: &mut App) {
    app.register_type::<WorldMap>()
        .register_type::<WorldGenerationSettings>()
        .register_type::<ChunkVisualisation>()
        .insert_resource(ClearColor(Color::srgb_u8(50, 45, 52)))
        .add_plugins(TilemapPlugin)
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
    fn new(entity: Entity, seed: u32) -> Self {
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

    /// Tries to fetch a block from world. Will return None, if the chunk doesn't exist or the block is of type BlockType::None
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

    /// Returns a result of type BlockType, if the corresponding chunk has been found. Returns an empty error, when the chunk is not loaded.
    pub fn get_raw_block(&self, coordinates: WorldCoordinates) -> Option<BlockType> {
        let (chunk_coordinate, block_coordinates) = coordinates.to_chunk_and_block();
        let index = to_index(block_coordinates);
        self.chunks
            .get(&chunk_coordinate.0)
            .map(|chunk| chunk.blocks[index])
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

fn spawn_world(world_generation_settings: Res<WorldGenerationSettings>, mut commands: Commands) {
    let entity = commands
        .spawn((
            Name::new("World Map"),
            // Transform::default(),
            // due to an issue with bevy_ecs_tilemap, we have to move the whole world by half a tile
            Transform::from_translation((-TILE_SIZE / 2.0).extend(0.0)),
            Visibility::Inherited,
        ))
        .id();
    commands.insert_resource(WorldMap::new(entity, world_generation_settings.seed));
}
