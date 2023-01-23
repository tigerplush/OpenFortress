use std::collections::HashMap;

use priority_queue::DoublePriorityQueue;
use bevy::prelude::*;

use crate::{position::Position, map::Map};

#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
#[derive(PartialEq)]
enum PathState {
    Queued,
    Calculating,
    Building,
    Success,
    Error,
}

#[derive(Component)]
pub struct Path {
    start: Position,
    target: Position,
    state: PathState,
    path: Vec<Position>,
    frontier: DoublePriorityQueue<Position, i32>,
    came_from: HashMap<Position, Option<Position>>,
    cost_so_far: HashMap<Position, i32>,
    current_lerp: f32,
}

impl Path {
    pub fn new(start: Position, target: Position) -> Self {
        Self {
            start,
            target,
            state: PathState::Queued,
            path: Vec::new(),
            frontier: DoublePriorityQueue::new(),
            came_from: HashMap::new(),
            cost_so_far: HashMap::new(),
            current_lerp: 0.0,
        }
    }
}

/// Calculates a path
/// todo: needs access to a world component/resource
/// maybe rework to attach PathState as components, so this can be split?
pub fn calculate_path(
    mut query: Query<&mut Path>,
    mut map: Res<Map>,
) {
    for mut path in &mut query {
        match path.state {
            PathState::Queued => {
                let start = path.start;
                info!("Started calculating path from {} to {}", start, path.target);
                path.frontier.push(start, 0);
                path.came_from.insert(start, None);
                path.cost_so_far.insert(start, 0);
                path.state = PathState::Calculating;
            },
            PathState::Calculating => {
                debug!("Calculating path from {} to {}...", path.start, path.target);
                match path.frontier.pop_min() {
                    None => {
                        error!("Could not calculate path from {} to {}", path.start, path.target);
                        path.state = PathState::Error;
                    },
                    Some((current, _prio)) => {
                        if current == path.target {
                            info!("Calculated path from {} to {}!", path.start, path.target);
                            path.state = PathState::Building;
                            return;
                        }

                        for neighbor in current.neighbors() {
                            if let Some(&cost) = path.cost_so_far.get(&current) {
                                // todo: replace 1 with cost of traversing a tile
                                let new_cost = cost + 1;
                                match (path.cost_so_far.get(&neighbor), new_cost + 1) {
                                    (None, prev) |
                                    (Some(&prev), _) if new_cost < prev => {
                                        path.cost_so_far.insert(neighbor, new_cost);
                                        // todo: finetune distance function, currently returns actual distance and then throws away decimals
                                        let priority = new_cost + neighbor.world_distance(path.target) as i32;
                                        debug!("Testing from {} to {}, cost {}, distance {}", current, neighbor, new_cost, neighbor.distance(path.target) as i32);
                                        path.frontier.push(neighbor, priority);
                                        path.came_from.insert(neighbor, Some(current));
                                    },
                                    _ => ()
                                }
                            }
                        }
                    }
                }
            },
            PathState::Building => {
                debug!("Rebuilding path...");
                if let Some(current) = path.path.last() {
                    if *current == path.start {
                        info!("{:?}", path.path);
                        if let Some(next) = path.path.pop() {
                            path.target = next;
                        }
                        path.state = PathState::Success;
                    }
                    else {
                        let previous = path.came_from.get(current).unwrap().unwrap();
                        debug!("Going from {} to {}", current, previous);
                        path.path.push(previous);
                    }
                }
                else {
                    let target = path.target;
                    debug!("Path is brand new, adding target {}", target);
                    path.path.push(target);
                }
            }
            _ => (),
        }
    }
}

const SPEED: f32 = 1.0;

/// System for Transforms to follow a path
/// todo: update dwarf position
pub fn follow_path(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Path)>,
){
    for (mut transform, mut path) in &mut query {
        if path.state != PathState::Success {
            return;
        }

        if transform.translation.distance(path.target.into_world()) < 0.1 {
            if let Some(next) = path.path.pop() {
                debug!("target and start are very close, next point");
                path.start = path.target;
                path.target = next;
                path.current_lerp = 0.0;
            }
            else {
                //despawn path
                debug!("No more points");
            }
        }
        else {
            let target: Vec3 = path.target.into_world();
            let start: Vec3 = path.start.into_world();
            path.current_lerp += time.delta_seconds() * SPEED;
            transform.translation = start.lerp(target, path.current_lerp) + Vec3::new(0.0, 0.5, 0.0);
            debug!("at {:?}, walking from {} towards {}", transform.translation, start, target);
        }
    }
}