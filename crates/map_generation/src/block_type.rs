use bevy::{color::palettes::css::{RED, WHITE}, ecs::relationship::RelatedSpawnerCommands, prelude::*};
use bevy_ecs_tilemap::{
    map::TilemapId,
    tiles::{TileBundle, TileColor, TileFlip, TilePos, TileStorage, TileTextureIndex},
};

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum BlockType {
    Solid(SolidMaterial),
    Liquid,
    None,
}

#[derive(Clone, Copy, PartialEq, Reflect)]
pub enum SolidMaterial {
    Dirt,
    Grass,
}

impl BlockType {
    pub(crate) fn is_solid(&self) -> bool {
        matches!(
            self,
            BlockType::Solid(_)
        )
    }

    pub(crate) fn spawn(
        &self,
        parent: &mut RelatedSpawnerCommands<'_, ChildOf>,
        x: u32,
        y: u32,
        target: Entity,
        tile_storage: &mut TileStorage,
        flags: u8,
    ) {
        for x_offset in 0..=1 {
            for y_offset in 0..=1 {
                let tile_pos = TilePos::new(x * 2 + x_offset, y * 2 + y_offset);
                let color = if tile_pos.x == 1 && tile_pos.y == 0 {
                    TileColor(RED.into())
                }
                else {
                    TileColor(self.color())
                };
                if let Some((texture_index, flip)) = self.texture_index(x_offset, y_offset, flags) {
                    let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(target),
                        color,
                        texture_index,
                        flip,
                        ..default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }
    }

    fn color(&self) -> Color {
        match self {
            BlockType::Solid(_) => Color::srgb_u8(223, 157, 117),
            _ => WHITE.into(),
        }
    }

    /// Finds the texture index and the flippage of a sprite
    fn texture_index(&self, x_offset: u32, y_offset: u32, flags: u8) -> Option<(TileTextureIndex, TileFlip)> {
        assert!( x_offset == 0 || x_offset == 1);
        assert!( y_offset == 0 || y_offset == 1);
        let tile_flip = TileFlip { x: x_offset == 1, y: y_offset == 0, d: false};
        if x_offset == 0 && y_offset == 0 {
            // bottom left
            // we are interested in: W, SW and S
            if flags & BOTTOM_LEFT_MASK == ISOLATED || flags & BOTTOM_LEFT_MASK == SOUTH_WEST {
                return Some((TileTextureIndex(3), tile_flip));
            }
            else if flags & BOTTOM_LEFT_MASK == WEST || flags & BOTTOM_LEFT_MASK == WEST | SOUTH_WEST {
                return Some((TileTextureIndex(2), tile_flip));
            }
            else if flags & BOTTOM_LEFT_MASK == SOUTH || flags & BOTTOM_LEFT_MASK == SOUTH | SOUTH_WEST {
                return Some((TileTextureIndex(1), tile_flip));
            }
            else if flags & BOTTOM_LEFT_MASK == SOUTH | WEST {
                return Some((TileTextureIndex(0), tile_flip));
            }
        }
        else if x_offset == 0 && y_offset == 1 {
            // upper left
            // we are interested in NW, N and W
            if flags & UPPER_LEFT_MASK == ISOLATED || flags & UPPER_LEFT_MASK == NORTH_WEST {
                return Some((TileTextureIndex(3), tile_flip));
            }
            else if flags & UPPER_LEFT_MASK == WEST || flags & UPPER_LEFT_MASK == WEST | NORTH_WEST {
                return Some((TileTextureIndex(2), tile_flip));
            }
            else if flags & UPPER_LEFT_MASK == NORTH || flags & UPPER_LEFT_MASK == NORTH | NORTH_WEST {
                return Some((TileTextureIndex(1), tile_flip));
            }
            else if flags & UPPER_LEFT_MASK == WEST | NORTH {
                return Some((TileTextureIndex(0), tile_flip));
            }
        }
        else if x_offset == 1 && y_offset == 0 {
            // bottom right
            // we are interested in: E, S and SE
            if flags & BOTTOM_RIGHT_MASK == ISOLATED || flags & BOTTOM_RIGHT_MASK == SOUTH_EAST {
                return Some((TileTextureIndex(3), tile_flip));
            }
            else if flags & BOTTOM_RIGHT_MASK == EAST || flags & BOTTOM_RIGHT_MASK == EAST | SOUTH_EAST {
                return Some((TileTextureIndex(2), tile_flip));
            }
            else if flags & BOTTOM_RIGHT_MASK == SOUTH || flags & BOTTOM_RIGHT_MASK == SOUTH | SOUTH_EAST {
                return Some((TileTextureIndex(1), tile_flip));
            }
            else if flags & BOTTOM_RIGHT_MASK == SOUTH | EAST {
                return Some((TileTextureIndex(0), tile_flip));
            }
        }
        else if x_offset == 1 && y_offset == 1 {
            // upper right
            // we are interested in N, NE and E
            if flags & UPPER_RIGHT_MASK == ISOLATED || flags & UPPER_LEFT_MASK == NORTH_EAST {
                return Some((TileTextureIndex(3), tile_flip));
            }
            else if flags & UPPER_RIGHT_MASK == EAST || flags & UPPER_RIGHT_MASK == EAST | NORTH_EAST {
                return Some((TileTextureIndex(2), tile_flip));
            }
            else if flags & UPPER_RIGHT_MASK == NORTH || flags & UPPER_RIGHT_MASK == NORTH | NORTH_EAST {
                return Some((TileTextureIndex(1), tile_flip));
            }
            else if flags & UPPER_RIGHT_MASK == EAST | NORTH {
                return Some((TileTextureIndex(0), tile_flip));
            }
        }
        None
    }
}


/// ```
///    SE S SW E W NE N NW
/// 0b  0 0  0 0 0  0 0  0
/// ```
const ISOLATED: u8 = 0b00000000;
const NORTH_WEST: u8 = 0b00000001;
const NORTH: u8 = 0b00000010;
const NORTH_EAST: u8 = 0b00000100;
const WEST: u8 = 0b00001000;
const EAST: u8 = 0b00010000;
const SOUTH_WEST: u8 = 0b00100000;
const SOUTH: u8 = 0b01000000;
const SOUTH_EAST: u8 = 0b10000000;

const BOTTOM_LEFT_MASK: u8 =  SOUTH | SOUTH_WEST | WEST;
const BOTTOM_RIGHT_MASK: u8 = SOUTH_EAST | SOUTH | EAST;
const UPPER_LEFT_MASK: u8 =  WEST | NORTH | NORTH_WEST;
const UPPER_RIGHT_MASK: u8 = EAST | NORTH_EAST | NORTH;