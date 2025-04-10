use std::cmp::Reverse;

use bevy::{platform_support::collections::HashMap, prelude::*};
use common::{
    functions::world_coordinates_to_world_position, traits::Neighbors, types::WorldCoordinates,
};
use map_generation::map_generation::WorldMap;
use path::Path;
use priority_queue::PriorityQueue;

pub mod path;

pub fn plugin(app: &mut App) {
    app.register_type::<Pathfinder>()
        .register_type::<Path>()
        .add_systems(Update, calculate_path.run_if(resource_exists::<WorldMap>))
        .add_systems(Update, (path::tick_path, path::follow_path).chain());
}

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
}

impl Pathfinder {
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
            steps: 0,
        }
    }

    fn calculate_step(&mut self, world_map: &WorldMap) -> PathfindingState {
        let Some((current_coordinates, _current_priority)) = self.frontier.pop() else {
            return PathfindingState::Failed;
        };

        if current_coordinates == self.target {
            return PathfindingState::Complete(Path::new(self.to_path()));
        }

        for (neighbor, neighbor_cost) in current_coordinates.all_neighbors() {
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

enum PathfindingState {
    Failed,
    Calculating,
    Complete(Path),
}

fn calculate_path(
    world_map: Res<WorldMap>,
    mut query: Query<(Entity, &mut Pathfinder)>,
    mut commands: Commands,
) {
    for (entity, mut path) in &mut query {
        match path.calculate_step(&world_map) {
            PathfindingState::Calculating => (),
            PathfindingState::Failed => {
                info!("pathfinding failed");
                commands.entity(entity).remove::<Pathfinder>();
            }
            PathfindingState::Complete(path) => {
                info!("pathfinding done");
                commands.entity(entity).remove::<Pathfinder>().insert(path);
            }
        }
    }
}
