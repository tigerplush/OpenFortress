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
        .add_systems(
            Update,
            (path::tick_path, path::follow_path, check_pathfinder).chain(),
        )
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
            PathfindingState::Failed(err) => match err {
                PathfindingErrors::NotEnoughChunks => {
                    info!("not enough chunks");
                }
                PathfindingErrors::Unreachable => {
                    info!("pathfinding failed");
                    commands.trigger(PathfindingCalculationEvent {
                        entity,
                        calculation: PathfindingCalculation::Failed,
                    });

                    commands.entity(entity).despawn();
                }
            },
            PathfindingState::Complete(path) => {
                info!("patfhinder {} done", entity);
                commands.trigger(PathfindingCalculationEvent {
                    entity,
                    calculation: PathfindingCalculation::Succeeded(path),
                });
                commands.entity(entity).despawn();
            }
        }
    }
}

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

#[derive(EntityEvent)]
#[entity_event(propagate)]
struct PathfindingCalculationEvent {
    entity: Entity,
    calculation: PathfindingCalculation,
}

#[derive(Debug, PartialEq)]
enum PathfindingCalculation {
    Failed,
    Succeeded(Path),
}

fn listen_for_path(
    trigger: On<PathfindingCalculationEvent>,
    listeners: Query<&PathfinderListener>,
    mut commands: Commands,
) {
    // if the event is triggered on a listener, we insert the path
    if let PathfindingCalculation::Succeeded(path) = &trigger.calculation {
        // if this is a successful path, we don't care about any other paths, so:
        // add path to the listener entity and remove listener
        if listeners.contains(trigger.entity) {
            debug!(
                "entity {} has found a successful path, removing all pathfinding children",
                trigger.entity
            );
            commands
                .entity(trigger.entity)
                .remove::<PathfinderListener>()
                .insert(path.clone());
        }
    }
}

fn check_pathfinder(
    listeners: Query<(Entity, Option<&Children>), With<PathfinderListener>>,
    pathfinders: Query<Entity, With<Pathfinder>>,
    mut commands: Commands,
) {
    for (parent, children) in listeners {
        if children.is_none_or(|x| {
            pathfinders
                .iter()
                .filter(|element| x.contains(element))
                .count()
                == 0
        }) {
            // no children, so remove dis shit
            debug!(
                "entity {} has no more Pathfinder children, removing listener...",
                parent
            );
            commands
                .entity(parent)
                .remove::<PathfinderListener>()
                .trigger(|entity| PathEvent {
                    entity,
                    state: PathState::CalculationFailed,
                });
        }
    }
}
