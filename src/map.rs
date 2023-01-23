use std::collections::HashMap;
use bevy::prelude::*;
use bevy_ecs_tilemap::{prelude::{TilemapSize, TilemapId, TilemapTileSize, TilemapTexture, get_tilemap_center_transform, TilemapType}, tiles::{TileStorage, TilePos, TileBundle, TileTextureIndex}, TilemapBundle};

use crate::position::Position;

#[derive(Clone, Copy)]
enum Tile {
    Solid,
    Empty,
}

#[derive(Resource)]
pub struct Map {
    width: i32,
    height: i32,
    depth: i32,
    tiles: HashMap<Position, Tile>,
}

impl Map {
    pub fn generate(
        width: i32,
        height: i32,
        depth: i32,
    ) -> Self {
        let mut tiles = HashMap::new();
        for d in -depth..=depth {
            for w in -width..=width {
                for h in -height..=height {
                    if d <= 0 {
                        tiles.insert(Position::new(w, h, d), Tile::Empty);
                    }
                    else {
                        tiles.insert(Position::new(w, h, d), Tile::Solid);
                    }
                }
            }
        }
        info!("Generated world with {} tiles", tiles.len());
        Self {
            width,
            height,
            depth,
            tiles,
        }
    }
}

pub fn spawn_map(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map: Res<Map>,
) {
    let texture_handle = asset_server.load("1_terrain.png");
    let map_size = TilemapSize {x: map.width as u32, y: map.height as u32};

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos {x, y};
            let tile_entity = commands
                .spawn(TileBundle {
                    texture_index: TileTextureIndex(16),
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .insert(Name::from(format!("Tile {:?}", tile_pos)))
                .set_parent(tilemap_entity)
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {x: 32.0, y: 32.0};
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity)
    .insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..default()
    })
    .insert(Name::from("Tilemap"));
}