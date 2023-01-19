use std::collections::HashMap;

use bevy::{prelude::*, log::LogPlugin};

use priority_queue::DoublePriorityQueue;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

mod position;
use position::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        ..default()
    }));
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(setup);
    app.add_startup_system(spawn_dwarf);
    app.add_startup_system(spawn_food);
    app.add_system(calculate_path);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Dwarf;

fn spawn_dwarf(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Dwarf)
        .insert(Position::ZERO)
        .insert(Path::new(Position::ZERO, Position::new(5, 5, 0)))
        .insert(Name::from("Dwarf"));
}

#[derive(Component)]
struct Food;

fn spawn_food(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Food)
        .insert(Position::new(5, 5, 0))
        .insert(Name::from("Food"));
}

enum PathState {
    Queued,
    Calculating,
    Success,
    Error,
}

#[derive(Component)]
struct Path {
    start: Position,
    target: Position,
    state: PathState,
    path: Option<Vec<Position>>,
    frontier: DoublePriorityQueue<Position, i32>,
    came_from: HashMap<Position, Option<Position>>,
    cost_so_far: HashMap<Position, i32>,
}

impl Path {
    pub fn new(start: Position, target: Position) -> Self {
        Self {
            start,
            target,
            state: PathState::Queued,
            path: None,
            frontier: DoublePriorityQueue::new(),
            came_from: HashMap::new(),
            cost_so_far: HashMap::new(),
        }
    }
}

/// Calculates a path
/// todo: needs access to a world component/resource
fn calculate_path(
    mut query: Query<&mut Path>
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
                            // todo: actually calculate path
                            path.state = PathState::Success;
                            return;
                        }

                        for neighbor in current.neighbors() {
                            if let Some(cost) = path.cost_so_far.get(&current) {
                                let new_cost = cost + 1;
                                path.cost_so_far.entry(neighbor).and_modify(|v| *v = new_cost).or_insert(new_cost);
                                let priority = new_cost + neighbor.distance(path.target) as i32;
                                debug!("Testing from {} to {}, cost {}, distance {}", current, neighbor, new_cost, neighbor.distance(path.target) as i32);
                                path.frontier.push(neighbor, priority);
                                path.came_from.insert(neighbor, Some(current));
                            }
                        }
                    }
                }
            },
            _ => (),
        }
    }
}
