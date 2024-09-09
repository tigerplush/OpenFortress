use std::collections::HashMap;

use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy::color::palettes::css::*;
use priority_queue::DoublePriorityQueue;

use crate::{
    map::{Map, Tile},
    position::Position,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (calculate_path, follow_path));
    #[cfg(feature = "debug")]
    app.add_systems(Update, draw_gizmos);
}

#[derive(PartialEq)]
pub enum PathState {
    Queued,
    Calculating,
    Building,
    Success,
    Error,
    Done,
}

#[derive(Component)]
pub struct Path {
    start: Position,
    target: Position,
    pub state: PathState,
    path: Vec<Position>,
    frontier: DoublePriorityQueue<Position, i32>,
    came_from: HashMap<Position, Option<Position>>,
    cost_so_far: HashMap<Position, i32>,
    current_lerp: f32,
    previous_position: Option<Position>,
    current_target: Option<Position>,
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
            previous_position: None,
            current_target: None,
        }
    }
}

/// Calculates a path
/// todo: needs access to a world component/resource
/// maybe rework to attach PathState as components, so this can be split?
pub fn calculate_path(map: Res<Map>, mut query: Query<&mut Path>) {
    for mut path in &mut query {
        match path.state {
            PathState::Queued => {
                let start = path.start;
                info!("Started calculating path from {} to {}", start, path.target);
                path.frontier.push(start, 0);
                path.came_from.insert(start, None);
                path.cost_so_far.insert(start, 0);
                path.state = PathState::Calculating;
            }
            PathState::Calculating => {
                debug!("Calculating path from {} to {}...", path.start, path.target);
                match path.frontier.pop_min() {
                    None => {
                        error!(
                            "Could not calculate path from {} to {}",
                            path.start, path.target
                        );
                        path.state = PathState::Error;
                    }
                    Some((current, _prio)) => {
                        if current == path.target {
                            info!("Calculated path from {} to {}!", path.start, path.target);
                            path.state = PathState::Building;
                            return;
                        }

                        for neighbor in current.neighbors() {
                            if map.get_tile(neighbor) == Tile::Grass {
                                continue;
                            }
                            if let Some(&cost) = path.cost_so_far.get(&current) {
                                // todo: replace 1 with cost of traversing a tile
                                let new_cost = cost + 1;
                                match (path.cost_so_far.get(&neighbor), new_cost + 1) {
                                    (None, prev) | (Some(&prev), _) if new_cost < prev => {
                                        path.cost_so_far.insert(neighbor, new_cost);
                                        // todo: finetune distance function, currently returns actual distance and then throws away decimals
                                        let priority =
                                            new_cost + neighbor.world_distance(path.target) as i32;
                                        debug!(
                                            "Testing from {} to {}, cost {}, distance {}",
                                            current,
                                            neighbor,
                                            new_cost,
                                            neighbor.distance(path.target) as i32
                                        );
                                        path.frontier.push(neighbor, priority);
                                        path.came_from.insert(neighbor, Some(current));
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
            PathState::Building => {
                debug!("Rebuilding path...");
                if let Some(current) = path.path.last() {
                    if *current == path.start {
                        info!("{:?}", path.path);
                        path.state = PathState::Success;
                    } else {
                        let previous = path.came_from.get(current).unwrap().unwrap();
                        debug!("Going from {} to {}", current, previous);
                        path.path.push(previous);
                    }
                } else {
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
pub fn follow_path(time: Res<Time>, mut query: Query<(&mut Transform, &mut Path)>) {
    for (mut transform, mut path) in &mut query {
        if path.state != PathState::Success {
            return;
        }

        if path.previous_position.is_none() {
            debug!("Setting first start");
            path.previous_position = Some(Position::from_world(transform.translation));
        }

        if path.current_target.is_none() {
            debug!("Setting first target");
            if let Some(next) = path.path.pop() {
                path.current_target = Some(next);
            } else {
                warn!("Something went wrong");
            }
        }

        if transform
            .translation
            .distance(path.current_target.unwrap().into_world())
            < 0.1
        {
            if let Some(next) = path.path.pop() {
                debug!("target and start are very close, next point");
                path.previous_position = path.current_target;
                path.current_target = Some(next);
                path.current_lerp = 0.0;
            } else {
                debug!("No more points");
                path.state = PathState::Done;
            }
        } else {
            let target: Vec3 = path.current_target.unwrap().into_world();
            let start: Vec3 = path.previous_position.unwrap().into_world();
            path.current_lerp = (path.current_lerp + time.delta_seconds() * SPEED).clamp(0.0, 1.0);
            transform.translation = start.lerp(target, path.current_lerp);
            debug!(
                "at {:?}, walking from {} towards {}",
                transform.translation, start, target
            );
        }
    }
}

#[cfg(feature = "debug")]
fn draw_gizmos(mut gizmos: Gizmos, query: Query<&Path>) {
    for path in &query {
        gizmos.line(path.start.into_world(), path.target.into_world(), YELLOW);

        if path.previous_position.is_some() && path.current_target.is_some() {
            gizmos.line(
                path.previous_position.unwrap().into_world(),
                path.current_target.unwrap().into_world(),
                ORANGE,
            );
        }

        if path.current_target.is_some() && path.path.len() > 0 {
            gizmos.line(
                path.current_target.unwrap().into_world(),
                path.path.last().unwrap().into_world(),
                GREEN,
            );
        }

        for i in 1..path.path.len() {
            let current = path.path[i - 1];
            let next = path.path[i];
            gizmos.line(current.into_world(), next.into_world(), GREEN);
        }
    }
}
