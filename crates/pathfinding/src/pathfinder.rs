use std::cmp::Reverse;

use bevy::{ecs::spawn::SpawnIter, platform::collections::HashMap, prelude::*};
use common::{traits::Neighbors, types::IWorldCoordinates};
use priority_queue::PriorityQueue;

use crate::{path::Path, pathfinding_map::PathfindingMap};

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
    cost_so_far: HashMap<IVec3, f32>,
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

/// The pathfinder listener will listen to all paths on children.
/// The first child, that collapses into a successful path will be copied onto the listener, thus following it.
/// If no pathfinders or paths are found, the pathfinding will be propagated as unsuccessful
#[derive(Component)]
pub(crate) struct PathfinderListener;

impl Pathfinder {
    /// Creates a new pathfinder that will try to find a path via A* from start to target
    fn new(start: IWorldCoordinates, target: IWorldCoordinates) -> Self {
        let frontier = PriorityQueue::from(vec![(start.0, Reverse(0))]);
        let mut came_from = HashMap::default();
        came_from.insert(start.0, None);
        let mut cost_so_far = HashMap::default();
        cost_so_far.insert(start.0, 0.0);
        Pathfinder {
            target: target.0,
            frontier,
            came_from,
            cost_so_far,
            ..default()
        }
    }

    /// Spawns a PathfinderListener with one separate Pathfinder child target the exact block
    ///
    /// Use this, if an entity has to land exactly on the given target
    pub fn exact(start: IWorldCoordinates, target: IWorldCoordinates) -> impl Bundle {
        (
            PathfinderListener,
            children![(
                Pathfinder::new(start, target),
                Name::new(format!("Pathfinder for {:?}", target.0))
            )],
        )
    }

    /// Spawns a PathfinderListener with separate Pathfinder children targeting the blocks surrounding the given target
    ///
    /// Use this if an entity has to come close to a given target but not go onto it
    pub fn nearest(start: IWorldCoordinates, target: IWorldCoordinates) -> impl Bundle {
        let finders: Vec<(Pathfinder, Name)> = target
            .same_layer_neighbors()
            .iter()
            .map(|(coordinates, _)| {
                (
                    Pathfinder::new(start, *coordinates),
                    Name::new(format!("Pathfinder for {:?}", coordinates.0)),
                )
            })
            .collect();
        (
            PathfinderListener,
            Children::spawn(SpawnIter(finders.into_iter())),
        )
    }

    pub(crate) fn calculate_step(
        &mut self,
        pathfinding_map: &impl PathfindingMap,
    ) -> PathfindingState {
        let Some((current_coordinates, _current_priority)) = self.frontier.pop() else {
            debug!("No frontier available");
            return PathfindingState::Failed(PathfindingErrors::Unreachable);
        };

        if current_coordinates == self.target {
            debug!("frontier is target");
            return PathfindingState::Complete(Path::new(self.to_path()));
        }

        for (neighbor, neighbor_cost) in pathfinding_map.get_neighbors(current_coordinates) {
            debug!(
                "current {:?} to neighbor {:?} would cost {}",
                current_coordinates, neighbor, neighbor_cost
            );
            let new_cost = self.cost_so_far.get(&current_coordinates).unwrap() + neighbor_cost;
            let current_cost = self.cost_so_far.get(&neighbor);
            if current_cost.is_none() || new_cost < *current_cost.unwrap() {
                self.cost_so_far.insert(neighbor, new_cost);
                let priority = new_cost + heuristic(neighbor, self.target);
                self.frontier
                    .push(neighbor, Reverse(priority.round() as u32));
                self.came_from.insert(neighbor, Some(current_coordinates));
            }
        }
        self.steps += 1;
        PathfindingState::Calculating
    }

    fn to_path(&self) -> Vec<IWorldCoordinates> {
        let mut points = vec![];
        let mut next = self.target;
        points.push(IWorldCoordinates(next));
        while let Some(point_option) = self.came_from.get(&next) {
            if let Some(point) = point_option {
                points.push(IWorldCoordinates(*point));
                next = *point;
            } else {
                break;
            }
        }
        points.reverse();
        points
    }
}

fn heuristic(from: IVec3, to: IVec3) -> f32 {
    from.distance_squared(to) as f32
}

pub(crate) enum PathfindingState {
    Failed(PathfindingErrors),
    Calculating,
    Complete(Path),
}

pub(crate) enum PathfindingErrors {
    Unreachable,
}
