use std::collections::HashMap;

use bevy::prelude::{*, system_adapter::new};

#[cfg(feature = "debug")]
use bevy::log::LogPlugin;

use priority_queue::DoublePriorityQueue;

#[cfg(feature = "inspector")]
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

#[cfg(feature = "fps")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod position;
use position::*;

fn main() {
    let mut app = App::new();

    #[cfg(not(feature = "debug"))]
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        ..default()
    }));
    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Path>();
    #[cfg(feature = "fps")]
    app.add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default());
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

#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
enum PathState {
    Queued,
    Calculating,
    Building,
    Success,
    Error,
}

#[cfg_attr(feature = "inspector", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
struct Path {
    start: Position,
    target: Position,
    state: PathState,
    path: Vec<Position>,
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
            path: Vec::new(),
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
                                        let priority = new_cost + neighbor.distance(path.target) as i32;
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
                        debug!("Path is finished, reversing...");
                        path.path.reverse();
                        info!("{:?}", path.path);
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
