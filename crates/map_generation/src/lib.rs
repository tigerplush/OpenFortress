use assets::tileset_asset::{TileType, TilesetAsset, TILE_SIZE};
use bevy::prelude::*;
use common::traits::AsVec2;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_world_map);
}

#[derive(Component)]
pub struct WorldMap;

fn on_add_world_map(
    trigger: Trigger<OnAdd, WorldMap>,
    tileset: Res<TilesetAsset>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.target())
        .insert((Transform::default(), Visibility::Inherited))
        .with_children(|world_map| {
            for x in 0..CHUNK_SIZE.x {
                for y in 0..CHUNK_SIZE.y {
                    world_map.spawn((
                        Sprite {
                            image: tileset.image.clone_weak(),
                            texture_atlas: Some(TextureAtlas {
                                layout: tileset.layout_handle.clone_weak(),
                                index: TileType::GRASS,
                            }),
                            ..default()
                        },
                        Transform::from_translation(((x, y).as_vec2() * TILE_SIZE).extend(-1.0)),
                    ));
                }
            }
        });
}

const CHUNK_SIZE: UVec2 = UVec2::new(16, 16);
