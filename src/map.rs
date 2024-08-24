use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::*,
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use leafwing_input_manager::prelude::*;
use noise::{Fbm, NoiseFn, Perlin, Seedable};
use std::collections::HashMap;

use crate::position::Position;

pub fn plugin(app: &mut App) {
    app.insert_resource(Map::generate(20, 20, 10))
        .add_plugins(InputManagerPlugin::<MapControls>::default())
        .add_systems(Startup, spawn_map)
        .add_systems(Update, update);
}

#[derive(Component)]
struct MapController {
    current_level: i32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum MapControls {
    LevelChange,
    ButtonPressed,
}

impl Actionlike for MapControls {
    fn input_control_kind(&self) -> leafwing_input_manager::InputControlKind {
        match self {
            MapControls::LevelChange => InputControlKind::Axis,
            _ => InputControlKind::Button,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Tile {
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
    pub fn generate(width: i32, height: i32, depth: i32) -> Self {
        let noise = Fbm::<Perlin>::default().set_seed(0);
        let mut tiles = HashMap::new();
        for d in -depth..=depth {
            for w in -width..=width {
                for h in -height..=height {
                    let height = noise.get([w as f64 + 0.5, h as f64 + 0.5]);
                    let height = height * 10.0;
                    // info!("height at {}, {} = {}", w, h, height);
                    if d <= height as i32 {
                        tiles.insert(Position::new(w, h, d), Tile::Solid);
                    } else {
                        tiles.insert(Position::new(w, h, d), Tile::Empty);
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

    pub fn get_tile(&self, x: u32, y: i32, z: u32) -> Tile {
        let w = x as i32 - self.width;
        let h = z as i32 - self.height;
        if let Some(&tile) = self.tiles.get(&Position::new(w, h, y)) {
            return tile;
        }
        Tile::Empty
    }
}

pub fn spawn_map(asset_server: Res<AssetServer>, mut commands: Commands, map: Res<Map>) {
    let texture_handle = asset_server.load("1_terrain_clone.png");
    let map_size = TilemapSize {
        x: (map.width * 2) as u32,
        y: (map.height * 2) as u32,
    };

    let input_map = InputMap::default().with_axis(
        MapControls::LevelChange,
        KeyboardVirtualAxis::VERTICAL_NUMPAD,
    )
    .with(MapControls::ButtonPressed, KeyCode::Numpad2)
    .with(MapControls::ButtonPressed, KeyCode::Numpad8);

    let current_level = 0;
    let parent = commands
        .spawn((
            MapController { current_level },
            InputManagerBundle::with_map(input_map),
            Name::from("Map"),
            SpatialBundle::default(),
        ))
        .id();

    for d in -map.depth..=current_level {
        spawn_layer(&mut commands, d, &map, &texture_handle, parent);
    }
}

fn spawn_layer(
    commands: &mut Commands,
    layer: i32,
    map: &Map,
    texture_handle: &Handle<Image>,
    parent: Entity,
) {
    let map_size = TilemapSize {
        x: (map.width * 2) as u32,
        y: (map.height * 2) as u32,
    };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            match map.get_tile(x, layer, y) {
                Tile::Solid => {
                    let tile_pos = TilePos { x, y };
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
                _ => {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = commands
                        .spawn(TileBundle {
                            texture_index: TileTextureIndex(15 * 24),
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            color: Color::srgba(0.0, 0.0, 0.0, 0.5).into(),
                            ..default()
                        })
                        .insert(Name::from(format!("Tile {:?}", tile_pos)))
                        .set_parent(tilemap_entity)
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size,
            transform: get_tilemap_center_transform(
                &map_size,
                &grid_size,
                &map_type,
                layer as f32 - 0.1,
            ),
            ..default()
        })
        .insert(Name::from(format!("Tilemap {}", layer)))
        .set_parent(parent);
}

fn update(asset_server: Res<AssetServer>, map: Res<Map>, mut query: Query<(&ActionState<MapControls>, &mut MapController, Entity)>, mut commands: Commands) {
    let Ok((action_state, mut controller, entity)) = query.get_single_mut() else {
        return;
    };

    if action_state.just_pressed(&MapControls::ButtonPressed) {
        let delta = action_state.value(&MapControls::LevelChange);
        controller.current_level += delta as i32;
        commands.entity(entity).despawn_descendants();
        let texture_handle = asset_server.load("1_terrain_clone.png");
        for d in controller.current_level-10..=controller.current_level {
            spawn_layer(&mut commands, d, &map, &texture_handle, entity);
        }
    }
}