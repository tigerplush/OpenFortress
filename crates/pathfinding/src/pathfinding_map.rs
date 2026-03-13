use bevy::prelude::*;
use common::{traits::Neighbors, types::IWorldCoordinates};
use map_generation::{block_type::BlockType, world_map::WorldMap};

pub(crate) trait PathfindingMap {
    fn get_neighbors(&self, coordinates: IVec3) -> impl Iterator<Item = (IVec3, f32)>;
}

impl PathfindingMap for WorldMap {
    fn get_neighbors(&self, coordinates: IVec3) -> impl Iterator<Item = (IVec3, f32)> {
        coordinates
            .all_neighbors()
            .into_iter()
            .filter_map(|(neighbor, squared_distance)| {
                let next_block = self.get_raw_block(IWorldCoordinates(neighbor))?;
                let block_below = self.get_raw_block(IWorldCoordinates(neighbor - IVec3::Z))?;
                if next_block != BlockType::None {
                    return None;
                }
                let BlockType::Solid(material) = block_below else {
                    return None;
                };

                Some((
                    neighbor,
                    material.traversal_cost() * squared_distance as f32,
                ))
            })
    }
}
