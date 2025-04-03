use bevy::prelude::*;

use crate::constants::TILE_SIZE;

pub fn world_to_tile(world_position: Vec3) -> IVec3 {
    let x = world_position.x / TILE_SIZE.x;
    let y = world_position.y / TILE_SIZE.y;
    let z = world_position.z;
    IVec3::new(x.round() as i32, y.round() as i32, z.round() as i32)
}
