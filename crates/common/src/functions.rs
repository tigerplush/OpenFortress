use bevy::prelude::*;

use crate::constants::TILE_SIZE;

pub fn world_to_tile(world_position: Vec3) -> IVec3 {
    let x = world_position.x / TILE_SIZE.x;
    let y = world_position.y / TILE_SIZE.y;
    let z = world_position.z;
    IVec3::new(x.round() as i32, y.round() as i32, z.round() as i32)
}

pub fn tile_to_world(tile_position: IVec3) -> Vec3 {
    let x = tile_position.x as f32 * TILE_SIZE.x;
    let y = tile_position.y as f32 * TILE_SIZE.y;
    let z = tile_position.z as f32;
    Vec3::new(x, y, z)
}
