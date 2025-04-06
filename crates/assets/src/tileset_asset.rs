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

impl BlockType {
    pub fn sprite(&self, tileset_asset: &TilesetAsset, flags: u8) -> Sprite {
        const OFFSET: usize = 0;
        let index: usize = match flags {
            0b00000000 => 0,
            0b00000001 => 0,
            0b00000010 => 1,
            0b00000011 => 1,
            0b00000100 => 0,
            0b00000101 => 0,
            0b00000110 => 1,
            0b00000111 => 1,
            0b00001000 => 2,
            0b00001001 => 2,
            0b00001010 => 3,
            0b00001011 => 4,
            0b00001100 => 2,
            0b00001101 => 2,
            0b00001110 => 3,
            0b00001111 => 4,
            0b00010000 => 5,
            0b00010001 => 5,
            0b00010010 => 6,
            0b00010011 => 6,
            0b00010100 => 5,
            0b00010101 => 5,
            0b00010110 => 7,
            0b00010111 => 7,
            0b00011000 => 8,
            0b00011001 => 8,
            0b00011010 => 9,
            0b00011011 => 10,
            0b00011100 => 8,
            0b00011101 => 8,
            0b00011110 => 11,
            0b00011111 => 12,
            0b00100000 => 0,
            0b00100001 => 0,
            _ => 0,
        };
        Sprite {
            image: tileset_asset.image.clone_weak(),
            texture_atlas: Some(TextureAtlas {
                layout: tileset_asset.layout_handle.clone_weak(),
                index: OFFSET + index,
            }),
            ..default()
        }
    }

    pub fn is_solid(&self) -> bool {
        match self {
            BlockType::BrightGrass | BlockType::Dirt | BlockType::Field | BlockType::Grass => true,
            _ => false,
        }
    }
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
    const PATH: &'static str = "tilesets/tileset.png";
}

impl FromWorld for TilesetAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 48, 1, None, None);
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
