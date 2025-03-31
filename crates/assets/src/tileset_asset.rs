use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}

pub const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum TileType {
    Grass,
    Water,
    Lava,
    BrightGrass,
    Dirt,
    Field,
    None,
}

impl From<usize> for TileType {
    fn from(value: usize) -> Self {
        match value {
            GRASS => TileType::Grass,
            WATER => TileType::Water,
            LAVA => TileType::Lava,
            BRIGHT_GRASS => TileType::BrightGrass,
            DIRT => TileType::Dirt,
            FIELD => TileType::Field,
            _ => TileType::None,
        }
    }
}

impl From<TileType> for usize {
    fn from(value: TileType) -> Self {
        match value {
            TileType::Grass => GRASS,
            TileType::Water => WATER,
            TileType::Lava => LAVA,
            TileType::BrightGrass => BRIGHT_GRASS,
            TileType::Dirt => DIRT,
            TileType::Field => FIELD,
            _ => NONE,
        }
    }
}

const GRASS: usize = 14 + 1;
const WATER: usize = 14 + 6;
const LAVA: usize = 14 + 11;
const BRIGHT_GRASS: usize = 10 * 14 + 1;
const DIRT: usize = 10 * 14 + 6;
const FIELD: usize = 10 * 14 + 11;
const NONE: usize = 23 * 14 + 13;

impl TilesetAsset {
    const PATH: &'static str = "1_terrain.png";
}

impl FromWorld for TilesetAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 14, 24, None, None);
            let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
            layouts.add(layout)
        };
        let assets = world.resource::<AssetServer>();
        TilesetAsset {
            image: assets.load_with_settings(
                TilesetAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            layout_handle,
        }
    }
}
