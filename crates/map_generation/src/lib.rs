use std::ops::{Range, RangeInclusive};

use assets::tileset_asset::{TILE_SIZE, TileType, TilesetAsset};
use bevy::{platform_support::collections::HashMap, prelude::*};
use common::{states::AppState, traits::AsVec2};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainGame), spawn_world)
        .add_systems(Update, (request_chunks, delete_chunks).run_if(in_state(AppState::MainGame)))
        .add_observer(on_add_chunk_visualisation);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct WorldMap {
    chunks: HashMap<IVec3, Chunk>,
}

impl WorldMap {
    fn new() -> Self {
        WorldMap {
            chunks: HashMap::default(),
        }
    }
}

fn spawn_world(mut commands: Commands) {
    commands.insert_resource(WorldMap::new());
}

fn on_add_chunk_visualisation(
    trigger: Trigger<OnAdd, ChunkVisualisation>,
    world_map: Res<WorldMap>,
    tileset: Res<TilesetAsset>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let chunk = chunks.get(trigger.target()).unwrap();
    if let Some(chunk) = world_map.chunks.get(&chunk.0) {
        commands.entity(trigger.target()).with_children(|parent| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    for z in (0..CHUNK_SIZE.z).rev() {
                        let index = to_index(x, y, z);
                        if chunk.blocks[index] != TileType::None {
                            parent.spawn((
                                Sprite {
                                    image: tileset.image.clone_weak(),
                                    texture_atlas: Some(TextureAtlas {
                                        layout: tileset.layout_handle.clone_weak(),
                                        index: chunk.blocks[index].into(),
                                    }),
                                    ..default()
                                },
                                Transform::from_translation(
                                    ((x, y).as_vec2() * TILE_SIZE).extend(-1.0),
                                ),
                            ));
                            break;
                        }
                    }
                }
            }
        });
    }
}

#[derive(Component)]
struct ChunkVisualisation(IVec3);

impl ChunkVisualisation {
    fn new(coordinates: IVec3) -> impl Bundle {
        (
            Name::new(format!("Chunk {}", coordinates)),
            ChunkVisualisation(coordinates),
            Transform::from_xyz(
                coordinates.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
                coordinates.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
                coordinates.z as f32 * CHUNK_SIZE.z as f32,
            ),
            Visibility::Inherited,
        )
    }
}

fn request_chunks(
    mut world_map: ResMut<WorldMap>,
    camera_transform: Single<(&Transform, &Projection)>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let Some((x_range, y_range, z_range)) =
        calculate_visible_chunk_ranges_from_single(camera_transform)
    else {
        return;
    };

    let mut requested_chunks = vec![];
    for x in x_range {
        for y in y_range.clone() {
            for z in z_range.clone() {
                requested_chunks.push(IVec3::new(x, y, z));
            }
        }
    }
    for coordinates in requested_chunks {
        // are the chunks already there?
        if let None = chunks.iter().find(|chunk| chunk.0 == coordinates) {
            // if not, spawn them
            info!("spawning chunk at {:?}", coordinates);
            let a = world_map
                .chunks
                .entry(coordinates)
                .or_insert(Chunk::new(coordinates));
            commands.spawn(ChunkVisualisation::new(coordinates));
        }
    }
}

fn delete_chunks(
    camera_transform: Single<(&Transform, &Projection)>,
    chunks: Query<(Entity, &ChunkVisualisation)>,
    mut commands: Commands,
) {
    let Some((x_range, y_range, z_range)) =
        calculate_visible_chunk_ranges_from_single(camera_transform)
    else {
        return;
    };
    for (entity, chunk) in &chunks {
        let coordinates = chunk.0;
        if !x_range.contains(&coordinates.x)
            || !y_range.contains(&coordinates.y)
            || !z_range.contains(&coordinates.z)
        {
            info!("despawing chunk at {:?}", coordinates);
            commands.entity(entity).despawn();
        }
    }
}

fn calculate_visible_chunk_ranges_from_single(
    camera_transform: Single<(&Transform, &Projection)>,
) -> Option<(Range<i32>, Range<i32>, RangeInclusive<i32>)> {
    let (transform, projection) = camera_transform.into_inner();
    let Projection::Orthographic(values) = projection else {
        return None;
    };
    Some(calculate_visible_chunk_ranges(transform, values))
}

/// Calculates which chunks are currently visible
fn calculate_visible_chunk_ranges(
    transform: &Transform,
    projection: &OrthographicProjection,
) -> (Range<i32>, Range<i32>, RangeInclusive<i32>) {
    let camera_x = transform.translation.x;
    let camera_y = transform.translation.y;

    let chunk_size_x = CHUNK_SIZE.x as f32 * TILE_SIZE.x;
    let chunk_size_y = CHUNK_SIZE.y as f32 * TILE_SIZE.y;
    let min_x = camera_x + projection.area.min.x;
    let max_x = camera_x + projection.area.max.x;
    let min_y = camera_y + projection.area.min.y;
    let max_y = camera_y + projection.area.max.y;

    let min_chunk_x = (min_x / chunk_size_x).floor() as i32;
    let max_chunk_x = (max_x / chunk_size_x).ceil() as i32;
    let min_chunk_y = (min_y / chunk_size_y).floor() as i32;
    let max_chunk_y = (max_y / chunk_size_y).ceil() as i32;
    (min_chunk_x..max_chunk_x, min_chunk_y..max_chunk_y, -1..=1)
}

const CHUNK_SIZE: UVec3 = UVec3::new(16, 16, 16);

#[derive(Reflect)]
struct Chunk {
    coordinates: IVec3,
    blocks: [TileType; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize],
}

impl Chunk {
    fn new(coordinates: IVec3) -> Self {
        let mut blocks = [TileType::None; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize];
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                for z in 0..CHUNK_SIZE.z {
                    let height = coordinates.z * CHUNK_SIZE.z as i32 + z as i32;
                    if height < 0 {
                        blocks[to_index(x, y, z)] = TileType::Grass
                    }
                }
            }
        }
        Chunk {
            coordinates,
            blocks,
        }
    }
}

fn to_index(x: u32, y: u32, z: u32) -> usize {
    (x * CHUNK_SIZE.x * CHUNK_SIZE.y + y * CHUNK_SIZE.z + z) as usize
}

#[test]
fn test_to_index() {
    let mut index = 0;
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            for z in 0..CHUNK_SIZE.z {
                assert_eq!(to_index(x, y, z), index);
                index += 1;
            }
        }
    }
}
