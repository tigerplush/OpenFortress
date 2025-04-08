use assets::tileset_asset::TilesetAsset;
use bevy::prelude::*;

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
    /// Tile is surrounded by nothing
    /// ```
    /// ?.?
    /// .O.
    /// ?.?
    /// ```
    const NONE: usize = 0;
    /// Tile is only adjacent to N
    /// ```
    /// ?O?
    /// .O.
    /// ?.?
    /// ```
    const N: usize = 1;
    /// Tile is only adjacent to E
    /// ```
    /// ?.?
    /// .OO
    /// ?.?
    /// ```
    const E: usize = 5;
    /// Tile is only adjacent to S
    /// ```
    /// ?.?
    /// .O.
    /// ?O?
    /// ```
    const S: usize = 13;
    /// Tile is only adjacent to W
    /// ```
    /// ?.?
    /// OO.
    /// ?.?
    /// ```
    const W: usize = 2;

    /// Tile is adjacent to N and E
    /// ```
    /// ?O.
    /// .OO
    /// ?.?
    /// ```
    const N_TO_E: usize = 6;
    /// Tile is adjacent to N and E
    /// ```
    /// .O?
    /// OO.
    /// ?.?
    /// ```
    const N_TO_W: usize = 3;
    /// Tile is adjacent to S and E
    /// ```
    /// ?.?
    /// .OO
    /// ?O.
    /// ```
    const S_TO_E: usize = 18;
    /// Tile is adjacent to N and E
    /// ```
    /// ?.?
    /// OO.
    /// .O?
    /// ```
    const S_TO_W: usize = 15;

    /// Tile is surrounded by W, N and E
    /// ```
    /// .O.
    /// OOO
    /// ?.?
    /// ```
    const T_UP: usize = 9;

    /// Tile is surrounded by N, E and S
    /// ```
    /// ?O.
    /// .OO
    /// ?O.
    /// ```
    const T_RIGHT: usize = 19;
    const T_DOWN: usize = 22;
    const T_LEFT: usize = 16;

    const TOP: usize = 42;
    const RIGHT: usize = 29;
    const BOTTOM: usize = 12;
    const LEFT: usize = 36;

    /// Tile is adjacent to W, NW and N
    const UPPER_LEFT_L: usize = 4;
    /// Tile is adjacent to N, NE, and E
    const UPPER_RIGHT_L: usize = 7;
    /// Tile is adjacent to E, SE and S
    const BOTTOM_RIGHT_L: usize = 35;
    /// Tile is adjacent to S, SW and W
    const BOTTOM_LEFT_L: usize = 27;

    const UPPER_LEFT_L_WITH_E: usize = 10;
    const UPPER_LEFT_L_WITH_S: usize = 17;
    const UPPER_RIGHT_L_WITH_S: usize = 21;
    const UPPER_RIGHT_L_WITH_W: usize = 11;
    const BOTTOM_LEFT_L_WITH_N: usize = 28;
    const BOTTOM_LEFT_L_WITH_E: usize = 30;
    const BOTTOM_RIGHT_L_WITH_W: usize = 37;

    /// ```
    /// OO.
    /// OOO
    /// .O.
    /// ```
    const UPPER_LEFT_L_WITH_E_AND_S: usize = 24;
    const UPPER_RIGHT_L_WITH_W_AND_S: usize = 25;
    const BOTTOM_RIGHT_L_WITH_W_AND_N: usize = 38;
    const BOTTOM_LEFT_L_WITH_E_AND_N: usize = 31;

    const TOP_WITH_N_EXIT: usize = 45;
    const BOTTOM_WITH_S_EXIT: usize = 26;
    const RIGHT_WITH_E_EXIT: usize = 32;
    const LEFT_WITH_W_EXIT: usize = 40;
    const UPPER_RIGHT_AND_BOTTOM_LEFT_L: usize = 33;
    const UPPER_LEFT_AND_BOTTOM_RIGHT_L: usize = 39;
    /// Tile is adjacent to N and S
    /// ```
    /// ?.?
    /// OOO
    /// ?.?
    /// ```
    const HORIZONTAL: usize = 8;
    /// Tile is adjacent to W and E
    /// ```
    /// ?O?
    /// .O.
    /// ?O?
    /// ```
    const VERTICAL: usize = 14;

    /// Tile is adjacent to cardinal directions
    /// ```
    /// ?O?
    /// OOO
    /// ?O?
    /// ```
    const CROSS: usize = 23;
    const NO_UPPER_LEFT_CORNER: usize = 44;
    const NO_UPPER_RIGHT_CORNER: usize = 43;
    const NO_BOTTOM_LEFT_CORNER: usize = 41;
    /// Tile is surrounded everywhere except bottom right
    /// ```
    /// OOO
    /// OOO
    /// OO.
    /// ```
    const NO_BOTTOM_RIGHT_CORNER: usize = 34;
    /// Tile is fully surrounded
    /// ```
    /// OOO
    /// OOO
    /// OOO
    /// ```
    const ALL: usize = 47;

    const MASKS: [(u8, u8, usize); 46] = [
        (0b01011010, 0b00000000, BlockType::NONE),
        (0b01011010, 0b00000010, BlockType::N),
        (0b01011010, 0b00010000, BlockType::E),
        (0b01011010, 0b00001000, BlockType::W),
        (0b01011010, 0b01000000, BlockType::S),
        (0b01011010, 0b00011000, BlockType::HORIZONTAL),
        (0b01011010, 0b01000010, BlockType::VERTICAL),
        (0b01011011, 0b00001010, BlockType::N_TO_W),
        (0b01011110, 0b00010010, BlockType::N_TO_E),
        (0b01111010, 0b01001000, BlockType::S_TO_W),
        (0b11011010, 0b01010000, BlockType::S_TO_E),
        (0b01011011, 0b00001011, BlockType::UPPER_LEFT_L),
        (0b01011110, 0b00010110, BlockType::UPPER_RIGHT_L),
        (0b11011010, 0b11010000, BlockType::BOTTOM_RIGHT_L),
        (0b01111010, 0b01101000, BlockType::BOTTOM_LEFT_L),
        (0b11111010, 0b11111000, BlockType::TOP),
        (0b01011111, 0b00011111, BlockType::BOTTOM),
        (0b01111011, 0b01101011, BlockType::RIGHT),
        (0b11011010, 0b11010010, BlockType::LEFT),
        (0b01111011, 0b01001011, BlockType::UPPER_LEFT_L_WITH_S),
        (0b11011110, 0b01010110, BlockType::UPPER_RIGHT_L_WITH_S),
        (0b01011111, 0b00011110, BlockType::UPPER_RIGHT_L_WITH_W),
        (0b01011111, 0b00011011, BlockType::UPPER_LEFT_L_WITH_E),
        (0b01111011, 0b01101010, BlockType::BOTTOM_LEFT_L_WITH_N),
        (0b11111010, 0b01111000, BlockType::BOTTOM_LEFT_L_WITH_E),
        (0b11111010, 0b11011000, BlockType::BOTTOM_RIGHT_L_WITH_W),
        (0b01011111, 0b00011010, BlockType::T_UP),
        (0b11011110, 0b01010010, BlockType::T_RIGHT),
        (0b11111010, 0b01011000, BlockType::T_DOWN),
        (0b01111011, 0b01001010, BlockType::T_LEFT),
        (0b11111111, 0b01011010, BlockType::CROSS),
        (0b11111111, 0b01011011, BlockType::UPPER_LEFT_L_WITH_E_AND_S),
        (
            0b11111111,
            0b01011110,
            BlockType::UPPER_RIGHT_L_WITH_W_AND_S,
        ),
        (
            0b11111111,
            0b11011010,
            BlockType::BOTTOM_RIGHT_L_WITH_W_AND_N,
        ),
        (
            0b11111111,
            0b01111010,
            BlockType::BOTTOM_LEFT_L_WITH_E_AND_N,
        ),
        (
            0b11111111,
            0b01111110,
            BlockType::UPPER_RIGHT_AND_BOTTOM_LEFT_L,
        ),
        (
            0b11111111,
            0b11011011,
            BlockType::UPPER_LEFT_AND_BOTTOM_RIGHT_L,
        ),
        (0b11111111, 0b11111010, BlockType::TOP_WITH_N_EXIT),
        (0b11111111, 0b01111011, BlockType::RIGHT_WITH_E_EXIT),
        (0b11111111, 0b01011111, BlockType::BOTTOM_WITH_S_EXIT),
        (0b11111111, 0b11011110, BlockType::LEFT_WITH_W_EXIT),
        (0b11111111, 0b11111110, BlockType::NO_UPPER_LEFT_CORNER),
        (0b11111111, 0b11111011, BlockType::NO_UPPER_RIGHT_CORNER),
        (0b11111111, 0b11011111, BlockType::NO_BOTTOM_LEFT_CORNER),
        (0b11111111, 0b01111111, BlockType::NO_BOTTOM_RIGHT_CORNER),
        (0b11111111, 0b11111111, BlockType::ALL),
    ];

    pub(crate) fn is_solid(&self) -> bool {
        matches!(
            self,
            BlockType::BrightGrass | BlockType::Dirt | BlockType::Field | BlockType::Grass
        )
    }

    pub(crate) fn sprite(&self, tileset: &TilesetAsset, flags: u8) -> Sprite {
        Sprite {
            image: tileset.image.clone_weak(),
            texture_atlas: Some(TextureAtlas {
                layout: tileset.layout_handle.clone_weak(),
                index: self.index(flags),
            }),
            ..default()
        }
    }

    fn index(&self, flag: u8) -> usize {
        for mask in Self::MASKS {
            let masked = flag & mask.0;
            if masked == mask.1 {
                return mask.2;
            }
        }
        panic!("flag {:08b} has no mapped index!", flag);
    }
}

#[test]
fn test_index_mapping() {
    for i in 0..255 {
        BlockType::Dirt.index(i);
    }
}
