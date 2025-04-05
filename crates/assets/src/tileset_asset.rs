use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum BlockType {
    Grass,
    Water,
    Lava,
    BrightGrass,
    Dirt,
    Field,
    None,
}

impl From<usize> for BlockType {
    fn from(value: usize) -> Self {
        match value {
            GRASS => BlockType::Grass,
            WATER => BlockType::Water,
            LAVA => BlockType::Lava,
            BRIGHT_GRASS => BlockType::BrightGrass,
            DIRT => BlockType::Dirt,
            FIELD => BlockType::Field,
            _ => BlockType::None,
        }
    }
}

impl From<BlockType> for usize {
    fn from(value: BlockType) -> Self {
        match value {
            BlockType::Grass => GRASS,
            BlockType::Water => WATER,
            BlockType::Lava => LAVA,
            BlockType::BrightGrass => BRIGHT_GRASS,
            BlockType::Dirt => DIRT,
            BlockType::Field => FIELD,
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
