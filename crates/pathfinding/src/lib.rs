use bevy::prelude::*;
use common::traits::AddNamedObserver;
use map_generation::map_generation::WorldMap;
use path::Path;
use pathfinder::{Pathfinder, PathfinderListener, PathfindingErrors, PathfindingState};

pub mod path;
pub mod pathfinder;

pub fn plugin(app: &mut App) {
    app.register_type::<Pathfinder>()
        .register_type::<Path>()
        .add_systems(Update, calculate_path.run_if(resource_exists::<WorldMap>))
        .add_systems(Update, (path::tick_path, path::follow_path).chain())
        .add_named_observer(listen_for_path, "listen_for_path");
}

fn calculate_path(
    world_map: Res<WorldMap>,
    mut query: Query<(Entity, &mut Pathfinder)>,
    mut commands: Commands,
) {
    for (entity, mut path) in &mut query {
        match path.calculate_step(&world_map) {
            PathfindingState::Calculating => (),
            PathfindingState::Failed(err) => {
                match err {
                    PathfindingErrors::NotEnoughChunks => {
                        info!("not enough chunks");
                    }
                    PathfindingErrors::Unreachable => {
                        info!("pathfinding failed");
                        commands.entity(entity).trigger(PathfindingEvent::Failed);
                    }
                }
            }
            PathfindingState::Complete(path) => {
                info!("pathfinding done");
                // commands.entity(entity).remove::<Pathfinder>().insert(path);
                commands.entity(entity).trigger(PathfindingEvent::Succeeded(path));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PathfindingEvent {
    Failed,
    Succeeded(Path),
}

impl Event for PathfindingEvent {
    type Traversal = &'static ChildOf;
    const AUTO_PROPAGATE: bool = true;
}

fn listen_for_path(trigger: Trigger<PathfindingEvent>, pathfinders: Query<&Pathfinder>, listeners: Query<&PathfinderListener>, mut commands: Commands) {
    // if the event is triggered on a listener, we insert the path
    if let PathfindingEvent::Succeeded(path) = trigger.event() {
        if listeners.contains(trigger.target()) {
            commands.entity(trigger.target()).insert(path.clone());
        }
    }

    // if this event is on the actual pathfinder object, we remove the pathfinder object since it's no longer needed
    if pathfinders.contains(trigger.target()) {
        commands.entity(trigger.target()).despawn();
    }
}