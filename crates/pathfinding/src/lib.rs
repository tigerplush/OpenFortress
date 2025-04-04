use std::cmp::Reverse;

use bevy::{platform_support::collections::HashMap, prelude::*};
use common::functions::tile_to_world;
use map_generation::WorldMap;
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
    pub fn new(start: IVec3, target: IVec3) -> Self {
        let frontier = PriorityQueue::from(vec![(start, Reverse(0))]);
        let mut came_from = HashMap::default();
        came_from.insert(start, None);
        let mut cost_so_far = HashMap::default();
        cost_so_far.insert(start, 0);
        Pathfinder {
            target,
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

        for (neighbor, neighbor_cost) in current_coordinates.neighbors() {
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
        points.push(tile_to_world(next));
        while let Some(point_option) = self.came_from.get(&next) {
            if let Some(point) = point_option {
                points.push(tile_to_world(*point));
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
            PathfindingState::Calculating => info!("pathfinding calculating"),
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

trait Neighbors<T> {
    /// Returns all neighbors with their squared cost.
    fn neighbors(&self) -> Vec<(T, u32)>;
}

#[rustfmt::skip]
impl Neighbors<IVec3> for IVec3 {
    fn neighbors(&self) -> Vec<(IVec3, u32)> {
        vec![
            (self + IVec3::new( 1,  0,  0), 1),
            (self + IVec3::new( 1,  1,  0), 2),
            (self + IVec3::new( 0,  1,  0), 1),
            (self + IVec3::new(-1,  1,  0), 2),
            (self + IVec3::new(-1,  0,  0), 1),
            (self + IVec3::new(-1, -1,  0), 2),
            (self + IVec3::new( 0, -1,  0), 1),
            (self + IVec3::new( 1, -1,  0), 2),
        ]
    }
}
