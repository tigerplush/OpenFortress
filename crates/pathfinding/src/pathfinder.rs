use std::cmp::Reverse;

use bevy::{platform_support::collections::HashMap, prelude::*};
use common::{
    functions::world_coordinates_to_world_position, traits::Neighbors, types::WorldCoordinates,
};
use map_generation::{block_type::BlockType, map_generation::WorldMap};
use priority_queue::PriorityQueue;

use crate::path::Path;

/// Attach this to calculate and ultimately follow a path.
///
/// Internally, Pathfinder uses IVec3 but these represent WorldCoordinates.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Pathfinder {
    target: IVec3,
    #[reflect(ignore)]
    frontier: PriorityQueue<IVec3, Reverse<u32>>,
    came_from: HashMap<IVec3, Option<IVec3>>,
    cost_so_far: HashMap<IVec3, u32>,
    steps: u32,
    allowed_failures: u8,
    current_failures: u8,
}

impl Default for Pathfinder {
    fn default() -> Self {
        Pathfinder {
            target: IVec3::ZERO,
            frontier: PriorityQueue::new(),
            came_from: HashMap::new(),
            cost_so_far: HashMap::new(),
            steps: 0,
            allowed_failures: 3,
            current_failures: 0,
        }
    }
}

impl Pathfinder {
    /// Creates a new pathfinder that will try to find a path via A* from start to target
    pub fn new(start: WorldCoordinates, target: WorldCoordinates) -> Self {
        let frontier = PriorityQueue::from(vec![(start.0, Reverse(0))]);
        let mut came_from = HashMap::default();
        came_from.insert(start.0, None);
        let mut cost_so_far = HashMap::default();
        cost_so_far.insert(start.0, 0);
        Pathfinder {
            target: target.0,
            frontier,
            came_from,
            cost_so_far,
            ..default()
        }
    }

    pub(crate) fn calculate_step(&mut self, world_map: &WorldMap) -> PathfindingState {
        let Some((current_coordinates, current_priority)) = self.frontier.pop() else {
            return PathfindingState::Failed(PathfindingErrors::Unreachable);
        };

        if current_coordinates == self.target {
            return PathfindingState::Complete(Path::new(self.to_path()));
        }

        for (neighbor, neighbor_cost) in current_coordinates.all_neighbors() {
            match self.is_floor_block(world_map, neighbor) {
                Ok(true) => (),
                Ok(false) => continue,
                Err(e) => {
                    self.current_failures += 1;
                    self.frontier.push(current_coordinates, current_priority);
                    return PathfindingState::Failed(e);
                }
            }

            let new_cost = self.cost_so_far.get(&current_coordinates).unwrap() + neighbor_cost;
            let current_cost = self.cost_so_far.get(&neighbor);
            if current_cost.is_none() || new_cost < *current_cost.unwrap() {
                self.cost_so_far.insert(neighbor, new_cost);
                let priority =
                    new_cost + heuristic(world_map) + neighbor.distance_squared(self.target) as u32;
                self.frontier.push(neighbor, Reverse(priority));
                self.came_from.insert(neighbor, Some(current_coordinates));
            }
        }
        self.steps += 1;
        PathfindingState::Calculating
    }

    fn is_floor_block(
        &self,
        world_map: &WorldMap,
        neighbor: IVec3,
    ) -> Result<bool, PathfindingErrors> {
        let neighbor_block = world_map.get_raw_block(WorldCoordinates(neighbor)).ok_or({
            if self.current_failures >= self.allowed_failures {
                PathfindingErrors::Unreachable
            } else {
                PathfindingErrors::NotEnoughChunks
            }
        })?;
        let block_below = world_map
            .get_raw_block(WorldCoordinates(neighbor - IVec3::NEG_Z))
            .ok_or({
                if self.current_failures >= self.allowed_failures {
                    PathfindingErrors::Unreachable
                } else {
                    PathfindingErrors::NotEnoughChunks
                }
            })?;
        Ok(neighbor_block == BlockType::None && matches!(block_below, BlockType::Solid(_)))
    }

    fn to_path(&self) -> Vec<Vec3> {
        let mut points = vec![];
        let mut next = self.target;
        points.push(world_coordinates_to_world_position(WorldCoordinates(next)));
        while let Some(point_option) = self.came_from.get(&next) {
            if let Some(point) = point_option {
                points.push(world_coordinates_to_world_position(WorldCoordinates(
                    *point,
                )));
                next = *point;
            } else {
                break;
            }
        }
        points.reverse();
        points
    }
}

fn heuristic(_world_map: &WorldMap) -> u32 {
    1
}

pub(crate) enum PathfindingState {
    Failed(PathfindingErrors),
    Calculating,
    Complete(Path),
}

pub(crate) enum PathfindingErrors {
    NotEnoughChunks,
    Unreachable,
}
