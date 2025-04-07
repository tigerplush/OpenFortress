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
            // 16
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
            // 32
            0b00100000 => 0,
            0b00100001 => 0,
            0b00100010 => 1,
            0b00100011 => 1,
            0b00100100 => 0,
            0b00100101 => 0,
            0b00100110 => 1,
            0b00100111 => 1,
            0b00101000 => 2,
            0b00101001 => 2,
            0b00101010 => 3,
            0b00101011 => 4,
            0b00101100 => 2,
            0b00101101 => 4,
            0b00101110 => 3,
            0b00101111 => 4,
            // 48
            0b00110000 => 5,
            0b00110001 => 5,
            0b00110010 => 6,
            0b00110011 => 6,
            0b00110100 => 5,
            0b00110101 => 5,
            0b00110110 => 7,
            0b00110111 => 7,
            0b00111000 => 8,
            0b00111001 => 8,
            0b00111010 => 9,
            0b00111011 => 10,
            0b00111100 => 8,
            0b00111101 => 8,
            0b00111110 => 11,
            0b00111111 => 12,
            // 64
            0b01000000 => 13,
            0b01000001 => 13,
            0b01000010 => 14,
            0b01000011 => 14,
            0b01000100 => 13,
            0b01000101 => 13,
            0b01000110 => 14,
            0b01000111 => 14,
            0b01001000 => 15,
            0b01001001 => 15,
            0b01001010 => 16,
            0b01001011 => 17,
            0b01001100 => 15,
            0b01001101 => 15,
            0b01001110 => 16,
            0b01001111 => 17,
            // 80
            0b01010000 => 18,
            0b01010001 => 18,
            0b01010010 => 19,
            0b01010011 => 19,
            0b01010100 => 20,
            0b01010101 => 20,
            0b01010110 => 21,
            0b01010111 => 21,
            0b01011000 => 22,
            0b01011001 => 22,
            0b01011010 => 23,
            0b01011011 => 24,
            0b01011100 => 22,
            0b01011101 => 22,
            0b01011110 => 25,
            0b01011111 => 26,
            // 96
            0b01100000 => 13,
            0b01100001 => 13,
            0b01100010 => 14,
            0b01100011 => 14,
            0b01100100 => 13,
            0b01100101 => 13,
            0b01100110 => 14,
            0b01100111 => 14,
            0b01101000 => 27,
            0b01101001 => 27,
            0b01101010 => 28,
            0b01101011 => 29,
            0b01101100 => 27,
            0b01101101 => 27,
            0b01101110 => 28,
            0b01101111 => 29,
            // 112
            0b01110000 => 20,
            0b01110001 => 20,
            0b01110010 => 19,
            0b01110011 => 19,
            0b01110100 => 20,
            0b01110101 => 20,
            0b01110110 => 21,
            0b01110111 => 21,
            0b01111000 => 30,
            0b01111001 => 30,
            0b01111010 => 31,
            0b01111011 => 32,
            0b01111100 => 30,
            0b01111101 => 30,
            0b01111110 => 33,
            0b01111111 => 34,
            // 128
            0b10000000 => 0,
            0b10000001 => 0,
            0b10000010 => 1,
            0b10000011 => 1,
            0b10000100 => 0,
            0b10000101 => 0,
            0b10000110 => 1,
            0b10000111 => 1,
            0b10001000 => 2,
            0b10001001 => 2,
            0b10001010 => 3,
            0b10001011 => 4,
            0b10001100 => 2,
            0b10001101 => 2,
            0b10001110 => 3,
            0b10001111 => 4,
            // 144
            0b10010000 => 5,
            0b10010001 => 5,
            0b10010010 => 6,
            0b10010011 => 6,
            0b10010100 => 5,
            0b10010101 => 5,
            0b10010110 => 7,
            0b10010111 => 7,
            0b10011000 => 8,
            0b10011001 => 8,
            0b10011010 => 9,
            0b10011011 => 10,
            0b10011100 => 8,
            0b10011101 => 8,
            0b10011110 => 11,
            0b10011111 => 12,
            // 160
            0b10100000 => 0,
            0b10100001 => 0,
            0b10100010 => 1,
            0b10100011 => 1,
            0b10100100 => 0,
            0b10100101 => 0,
            0b10100110 => 1,
            0b10100111 => 1,
            0b10101000 => 2,
            0b10101001 => 2,
            0b10101010 => 3,
            0b10101011 => 4,
            0b10101100 => 2,
            0b10101101 => 2,
            0b10101110 => 3,
            0b10101111 => 4,
            // 176
            0b10110000 => 5,
            0b10110001 => 5,
            0b10110010 => 6,
            0b10110011 => 6,
            0b10110100 => 5,
            0b10110101 => 5,
            0b10110110 => 7,
            0b10110111 => 7,
            0b10111000 => 8,
            0b10111001 => 8,
            0b10111010 => 9,
            0b10111011 => 10,
            0b10111100 => 8,
            0b10111101 => 8,
            0b10111110 => 11,
            0b10111111 => 12,
            // 192
            0b11000000 => 13,
            0b11000001 => 13,
            0b11000010 => 14,
            0b11000011 => 14,
            0b11000100 => 13,
            0b11000101 => 13,
            0b11000110 => 14,
            0b11000111 => 14,
            0b11001000 => 15,
            0b11001001 => 15,
            0b11001010 => 16,
            0b11001011 => 17,
            0b11001100 => 15,
            0b11001101 => 15,
            0b11001110 => 16,
            0b11001111 => 17,
            // 208
            0b11010000 => 35,
            0b11010001 => 35,
            0b11010010 => 36,
            0b11010011 => 36,
            0b11010100 => 35,
            0b11010101 => 35,
            0b11010110 => 36,
            0b11010111 => 36,
            0b11011000 => 37,
            0b11011001 => 37,
            0b11011010 => 38,
            0b11011011 => 39,
            0b11011100 => 37,
            0b11011101 => 37,
            0b11011110 => 40,
            0b11011111 => 41,
            // 224
            0b11100000 => 13,
            0b11100001 => 13,
            0b11100010 => 14,
            0b11100011 => 14,
            0b11100100 => 13,
            0b11100101 => 13,
            0b11100110 => 14,
            0b11100111 => 14,
            0b11101000 => 27,
            0b11101001 => 27,
            0b11101010 => 16,
            0b11101011 => 29,
            0b11101100 => 27,
            0b11101101 => 27,
            0b11101110 => 28,
            0b11101111 => 29,
            // 240
            0b11110000 => 35,
            0b11110001 => 35,
            0b11110010 => 36,
            0b11110011 => 36,
            0b11110100 => 35,
            0b11110101 => 35,
            0b11110110 => 36,
            0b11110111 => 36,
            0b11111000 => 42,
            0b11111001 => 42,
            0b11111010 => 38,
            0b11111011 => 43,
            0b11111100 => 42,
            0b11111101 => 42,
            0b11111110 => 44,
            0b11111111 => 47,
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
        matches!(self, BlockType::BrightGrass | BlockType::Dirt | BlockType::Field | BlockType::Grass)
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
