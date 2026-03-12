use bevy::prelude::*;
use path::Path;

pub mod path;
pub mod pathfinder;
mod pathfinding;
pub mod pathfinding_map;
pub use pathfinding::plugin;

#[derive(EntityEvent)]
pub struct PathEvent {
    pub entity: Entity,
    pub state: PathState,
}

#[derive(Event)]
pub enum PathState {
    CalculationFailed,
    Completed,
}

#[derive(Debug, EntityEvent)]
#[entity_event(auto_propagate)]
struct PathfindingCalculationEvent {
    entity: Entity,
    calculation: PathfindingCalculation,
}

#[derive(Debug, PartialEq)]
enum PathfindingCalculation {
    Failed,
    Succeeded(Path),
}
