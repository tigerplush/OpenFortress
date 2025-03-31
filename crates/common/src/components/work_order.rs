use bevy::prelude::*;

use crate::{constants::TILE_SIZE, functions::world_to_tile};

#[derive(Clone, Component, Copy, PartialEq, Reflect)]
pub enum WorkOrder {
    Dig(IVec3),
}

impl WorkOrder {
    pub fn dig(world_position: Vec3) -> impl Bundle {
        let tile_coordinates = world_to_tile(world_position);
        (
            Name::new(format!("WorkOrder - Dig {}", tile_coordinates)),
            Transform::from_translation((world_position / TILE_SIZE.extend(1.0)).round() * TILE_SIZE.extend(1.0)),
            WorkOrder::Dig(tile_coordinates),
        )
    }
}