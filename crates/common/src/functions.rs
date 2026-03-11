use bevy::prelude::*;

use crate::{constants::TILE_SIZE, types::IWorldCoordinates};

pub fn world_position_to_world_coordinates(world_position: Vec3) -> IWorldCoordinates {
    let x = world_position.x / TILE_SIZE.x;
    let y = world_position.y / TILE_SIZE.y;
    let z = world_position.z;
    IWorldCoordinates(IVec3::new(
        x.round() as i32,
        y.round() as i32,
        z.round() as i32,
    ))
}

pub fn world_coordinates_to_world_position(world_coordinates: IWorldCoordinates) -> Vec3 {
    let x = world_coordinates.0.x as f32 * TILE_SIZE.x;
    let y = world_coordinates.0.y as f32 * TILE_SIZE.y;
    let z = world_coordinates.0.z as f32;
    Vec3::new(x, y, z)
}
