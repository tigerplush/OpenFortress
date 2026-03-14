use std::{collections::HashMap, hash::Hash};

use bevy::{color::palettes::css::WHITE, ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use common::{constants::TILE_SIZE, traits::Neighbors, types::IWorldCoordinates};

use crate::{block_type::BlockType, chunk::CHUNK_SIZE, world_map::WorldMap};

#[derive(Eq, Hash, PartialEq)]
pub(crate) enum TileType {
    Full,
    Half,
    Animated,
    Fog,
}

impl TileType {
    /// Returns the tilemap size based on its type
    pub(crate) fn tilemap_size(&self) -> UVec2 {
        match *self {
            TileType::Half => CHUNK_SIZE.truncate() * 2,
            _ => CHUNK_SIZE.truncate(),
        }
    }

    /// Returns the tile size based on its type
    pub(crate) fn tile_size(&self) -> Vec2 {
        match *self {
            TileType::Half => TILE_SIZE / 2.0,
            _ => TILE_SIZE,
        }
    }

    /// Returns the tilemap name based on its type
    pub(crate) fn name(&self) -> impl Into<String> {
        match *self {
            TileType::Full => "Half-Tile Tilemap",
            TileType::Half => "Full-Tile Tilemap",
            TileType::Animated => "Animated Tilemap",
            TileType::Fog => "Fog Tilemap",
        }
    }

    /// Returns the tilemap offset based on its type
    pub(crate) const fn z_offset(&self) -> f32 {
        match *self {
            TileType::Full => -0.5,
            TileType::Half => 0.0,
            TileType::Animated => -0.5,
            TileType::Fog => 1.0,
        }
    }
}

/// Wrapper to wrap the contents of a full-tile tilemap
pub(crate) enum TileWrapper {
    /// Wrapper for a full block type
    Full(BlockType),
    /// Wrapper for a half block type
    Half((BlockType, u8)),
    /// Wrapper for an animated tile type
    Animated,
    /// Wrapper for a fog type, carries the opacity in percent (ranging 0.0 to 1.0)
    Fog(f32),
}

impl TileWrapper {
    /// Spaws a tilebundle based on its type.
    ///
    /// For half-tiles, this actually spawns four tiles.
    pub(crate) fn spawn_bundle(
        &self,
        parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
        tile_pos_type: TilePosType,
        tilemap_id: TilemapId,
        tile_storage: &mut TileStorage,
    ) {
        match (self, tile_pos_type) {
            (TileWrapper::Full(block), TilePosType::Full(position)) => {
                let tile_entity = parent
                    .spawn(TileBundle {
                        position,
                        tilemap_id,
                        ..block.floor_tile()
                    })
                    .id();
                tile_storage.set(&position, tile_entity);
            }
            (TileWrapper::Half((block_type, flags)), TilePosType::Half((x, y))) => {
                block_type.spawn_half_tile(parent, x, y, tilemap_id, tile_storage, *flags);
            }
            (TileWrapper::Animated, TilePosType::Full(position)) => {
                let tile_entity = parent
                    .spawn((
                        TileBundle {
                            position,
                            tilemap_id,
                            texture_index: TileTextureIndex(0),
                            ..default()
                        },
                        AnimatedTile {
                            start: 0,
                            end: 8,
                            speed: 0.1,
                        },
                    ))
                    .id();
                tile_storage.set(&position, tile_entity);
            }
            (TileWrapper::Fog(opacity), TilePosType::Full(position)) => {
                let tile_entity = parent
                    .spawn(TileBundle {
                        position,
                        tilemap_id,
                        color: TileColor(WHITE.with_alpha(*opacity).into()),
                        ..default()
                    })
                    .id();
                tile_storage.set(&position, tile_entity);
            }
            _ => (),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(crate) enum TilePosType {
    Full(TilePos),
    Half((u32, u32)),
}

/// Type to simplyfile Tilemap handling
pub(crate) type Tilemap = HashMap<TilePosType, TileWrapper>;

/// Contains all relevant tilemaps
pub(crate) struct Tilemaps(pub(crate) HashMap<TileType, Tilemap>);

impl Default for Tilemaps {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(TileType::Full, HashMap::new());
        map.insert(TileType::Half, HashMap::new());
        map.insert(TileType::Animated, HashMap::new());
        map.insert(TileType::Fog, HashMap::new());
        Tilemaps(map)
    }
}

pub(crate) trait ToTiles {
    /// Adds itself to the tilemap.
    ///
    /// Returns true if the tile is solid.
    fn to_tile(
        &self,
        pos: UVec2,
        z: i32,
        tilemaps: &mut Tilemaps,
        current_world_coordinates: &IWorldCoordinates,
        world_map: &WorldMap,
        visible_layers: f32,
    ) -> bool;
}

impl ToTiles for BlockType {
    fn to_tile(
        &self,
        pos: UVec2,
        z: i32,
        tilemaps: &mut Tilemaps,
        current_world_coordinates: &IWorldCoordinates,
        world_map: &WorldMap,
        visible_layers: f32,
    ) -> bool {
        match self {
            BlockType::Solid(_) if z == 0 => {
                let mut flags = 0;
                for (index, (neighbor, _)) in current_world_coordinates
                    .same_layer_neighbors()
                    .iter()
                    .enumerate()
                {
                    // fetch the block
                    // check if its solid
                    let solid: u8 = world_map.solidness(*neighbor).into();
                    // add its state to the flag
                    flags |= solid << index;
                }
                tilemaps.0.entry(TileType::Half).and_modify(|m| {
                    m.insert(
                        TilePosType::Half((pos.x, pos.y)),
                        TileWrapper::Half((*self, flags)),
                    );
                });
                true
            }
            BlockType::Solid(_) => {
                tilemaps.0.entry(TileType::Full).and_modify(|m| {
                    m.insert(
                        TilePosType::Full(TilePos::new(pos.x, pos.y)),
                        TileWrapper::Full(*self),
                    );
                });
                true
            }
            BlockType::Liquid => {
                tilemaps.0.entry(TileType::Animated).and_modify(|m| {
                    m.insert(
                        TilePosType::Full(TilePos::new(pos.x, pos.y)),
                        TileWrapper::Animated,
                    );
                });
                true
            }
            BlockType::None if z != 0 => {
                tilemaps.0.entry(TileType::Fog).and_modify(|m| {
                    m.insert(
                        TilePosType::Full(TilePos::new(pos.x, pos.y)),
                        TileWrapper::Fog(z as f32 / visible_layers),
                    );
                });
                false
            }
            BlockType::None => false,
        }
    }
}
