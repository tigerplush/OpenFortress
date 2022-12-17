use std::fmt::{self, Display, Formatter};
use std::{collections::HashMap, hash::Hash};
use std::ops::*;

use bevy::prelude::*;
use bevy::log;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

use rand::{{thread_rng, Rng}};

use priority_queue::DoublePriorityQueue;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Component, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub elevation: i16,
}

pub struct Path(Vec<Position>);

const DIRECTIONS: [Position; 6] = [
    // up
    Position { x: 0, y: 0, elevation: 1},
    // down
    Position { x: 0, y: 0, elevation: -1},
    // left
    Position { x: -1, y: 0, elevation: 0},
    // right
    Position { x: 1, y: 0, elevation: 0},
    // forward
    Position { x: 0, y: 1, elevation: 0},
    // backward
    Position { x: 0, y: -1, elevation: 0}
];

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Position { x: self.x + other.x, y: self.y + other.y, elevation: self.elevation + other.elevation }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.elevation)
    }
}

impl Position {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Position {
            x: rng.gen_range(-10..10),
            y: rng.gen_range(-10..10),
            elevation: rng.gen_range(-10..10)
        }
    }

    fn neighbors(self) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        for direction in DIRECTIONS {
            result.push(direction + self);
        }
        result
    }

    fn distance(self, other: Self) -> i32 {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        let elevation_diff = self.elevation - other.elevation;
        [x_diff.abs(), y_diff.abs(), elevation_diff.abs()].iter().copied().max().unwrap().into()
    }

    pub fn calculate_path(
        start: Position,
        target: Position
    ) -> Option<Path> {
        log::info!("Calculating path from {} to {}", start, target);
        let mut priority_queue: DoublePriorityQueue<Position, i32> = DoublePriorityQueue::new();
        priority_queue.push(start, 0);

        let mut came_from: HashMap<Position, Option<Position>> = HashMap::new();
        came_from.insert(start, None);

        let mut cost_so_far: HashMap<Position, i32> = HashMap::new();
        cost_so_far.insert(start, 0);

        while priority_queue.len() > 0 {
            if let Some((current, _prio)) = priority_queue.pop_min() {
                if current == target {
                    log::info!("Found path: {}", current == target);
                    return Some(Path(Vec::new()));
                }

                for neighbor in current.neighbors() {
                    if let Some(cost) = cost_so_far.get(&current) {
                        let new_cost = cost + 1;
                        cost_so_far.entry(neighbor).and_modify(|v| *v = new_cost).or_insert(new_cost);
                        let priority = new_cost + neighbor.distance(target);
                        priority_queue.push(neighbor, priority);
                        came_from.insert(neighbor, Some(current));
                    }
                }
            }
        }
        None
    }
}