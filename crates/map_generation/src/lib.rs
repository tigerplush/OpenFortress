use std::ops::{Range, RangeInclusive};

use assets::tileset_asset::{TileType, TilesetAsset};
use bevy::{platform_support::collections::HashMap, prelude::*};
use camera::CameraLayer;
use common::{constants::TILE_SIZE, states::AppState, traits::AsVec2};
use noise::{NoiseFn, OpenSimplex};

pub fn plugin(app: &mut App) {
    app.register_type::<WorldMap>()
        .register_type::<ChunkVisualisation>()
        .add_systems(OnEnter(AppState::MainGame), spawn_world)
        .add_systems(
            Update,
            (request_chunks, delete_chunks).run_if(in_state(AppState::MainGame)),
        )
        .add_observer(on_add_chunk_visualisation);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WorldMap {
    chunks: HashMap<IVec3, Chunk>,
    #[reflect(ignore)]
    noise: OpenSimplex,
}

impl WorldMap {
    fn new() -> Self {
        WorldMap {
            chunks: HashMap::default(),
            noise: OpenSimplex::new(0),
        }
    }

    fn get_chunk(&mut self, coordinates: IVec3) -> &Chunk {
        self.chunks
            .entry(coordinates)
            .or_insert(Chunk::new(coordinates, self.noise))
    }

    pub fn get_tile(&self, coordinates: IVec3) -> Option<TileType> {
        let chunk_coordinates = coordinates.div_euclid(CHUNK_SIZE.as_ivec3());
        let block_coordinates = coordinates.rem_euclid(CHUNK_SIZE.as_ivec3()).as_uvec3();
        let index = to_index(
            block_coordinates.x,
            block_coordinates.y,
            block_coordinates.z,
        );
        self.chunks
            .get(&chunk_coordinates)
            .and_then(|chunk| match chunk.blocks[index] {
                TileType::None => None,
                _ => Some(chunk.blocks[index])
            })
    }
}

fn spawn_world(mut commands: Commands) {
    commands.insert_resource(WorldMap::new());
}

fn on_add_chunk_visualisation(
    trigger: Trigger<OnAdd, ChunkVisualisation>,
    mut world_map: ResMut<WorldMap>,
    tileset: Res<TilesetAsset>,
    chunks: Query<&ChunkVisualisation>,
    mut commands: Commands,
) {
    let chunk_visualisation = chunks.get(trigger.target()).unwrap();
    let chunk = world_map.get_chunk(chunk_visualisation.0);
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

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ChunkVisualisation(IVec3);

impl ChunkVisualisation {
    fn bundle(coordinates: IVec3) -> impl Bundle {
        (
            Name::new(format!("Chunk {}", coordinates)),
            ChunkVisualisation(coordinates),
            Transform::from_xyz(
                coordinates.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
                coordinates.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
                coordinates.z as f32,
            ),
            Visibility::Inherited,
        )
    }
}

fn request_chunks(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
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
        if !chunks.iter().any(|chunk| chunk.0 == coordinates) {
            // if not, spawn them
            commands.spawn(ChunkVisualisation::bundle(coordinates));
        }
    }
}

fn delete_chunks(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
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
            commands.entity(entity).despawn();
        }
    }
}

fn calculate_visible_chunk_ranges_from_single(
    camera_transform: Single<(&Transform, &CameraLayer, &Projection)>,
) -> Option<(Range<i32>, Range<i32>, RangeInclusive<i32>)> {
    let (transform, layer, projection) = camera_transform.into_inner();
    let Projection::Orthographic(values) = projection else {
        return None;
    };
    Some(calculate_visible_chunk_ranges(transform, layer, values))
}

/// Calculates which chunks are currently visible
fn calculate_visible_chunk_ranges(
    transform: &Transform,
    layer: &CameraLayer,
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
    (
        min_chunk_x..max_chunk_x,
        min_chunk_y..max_chunk_y,
        (layer.0 - 1)..=(layer.0 + 1),
    )
}

const CHUNK_SIZE: UVec3 = UVec3::new(16, 16, 1);

#[derive(Reflect)]
struct Chunk {
    coordinates: IVec3,
    blocks: [TileType; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize],
}

impl Chunk {
    fn new(coordinates: IVec3, noise: OpenSimplex) -> Self {
        let mut blocks = [TileType::None; (CHUNK_SIZE.x * CHUNK_SIZE.y * CHUNK_SIZE.z) as usize];
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let world_x = coordinates.x as f32 + (x as f32 / CHUNK_SIZE.x as f32);
                let world_y = coordinates.y as f32 + (y as f32 / CHUNK_SIZE.y as f32);
                let threshold = noise
                    .get([world_x as f64, world_y as f64])
                    .remap(-1.0, 1.0, -10984.0, 8848.0)
                    .round() as i32;
                for z in 0..CHUNK_SIZE.z {
                    let height = coordinates.z * CHUNK_SIZE.z as i32 + z as i32;
                    let tile_type = if height == threshold && threshold > 0 {
                        TileType::BrightGrass
                    } else if height < threshold {
                        TileType::Dirt
                    } else if height > threshold && height < 0 {
                        TileType::Water
                    } else {
                        TileType::None
                    };
                    blocks[to_index(x, y, z)] = tile_type;
                }
            }
        }
        Chunk {
            coordinates,
            blocks,
        }
    }
}

/// returns the index of a tile in it's block array by coordinates
fn to_index(x: u32, y: u32, z: u32) -> usize {
    (x * CHUNK_SIZE.y * CHUNK_SIZE.z + y * CHUNK_SIZE.z + z) as usize
}

pub fn world_to_tile(world_position: Vec3) -> IVec3 {
    let x = world_position.x / TILE_SIZE.x;
    let y = world_position.y / TILE_SIZE.y;
    let z = world_position.z;
    IVec3::new(x.round() as i32, y.round() as i32, z.round() as i32)
}

#[test]
fn test_to_index() {
    let mut index = 0;
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            for z in 0..CHUNK_SIZE.z {
                assert_eq!(to_index(x, y, z), index, "x: {}, y: {}, z: {}", x, y, z);
                index += 1;
            }
        }
    }
}
