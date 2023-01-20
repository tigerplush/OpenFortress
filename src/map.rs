use std::collections::HashMap;
use bevy::prelude::*;

use crate::position::Position;

enum Tile {
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
    pub fn generate(
        width: i32,
        height: i32,
        depth: i32,
    ) -> Self {
        let mut tiles = HashMap::new();
        for d in -depth..depth {
            for w in -width..width {
                for h in -height..height {
                    if d <= 0 {
                        tiles.insert(Position::new(w, h, d), Tile::Empty);
                    }
                    else {
                        tiles.insert(Position::new(w, h, d), Tile::Solid);
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
}