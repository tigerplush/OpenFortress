use bevy::prelude::*;
use common::{
    functions::world_position_to_world_coordinates, traits::SpawnNamedObserver,
    types::BlockCoordinates,
};
use pathfinding::{PathEvent, PathState, pathfinder::Pathfinder};

use crate::tasks::TaskEvent;

use super::{Task, TaskState};

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct WalkToNearest(pub(crate) BlockCoordinates);

pub(crate) fn handle(query: Query<(Entity, &Transform, &WalkToNearest)>, mut commands: Commands) {
    for (entity, transform, walk_to) in &query {
        info!("inserting pathfinding component");
        let start = world_position_to_world_coordinates(transform.translation);
        let target = commands
            .entity(entity)
            .remove::<WalkToNearest>()
            .insert(Pathfinder::nearest(start, walk_to.0))
            .id();
        commands.spawn_named_observer(target, on_path_event, "on_path_event");
    }
}

fn on_path_event(trigger: On<PathEvent>, mut commands: Commands) {
    match trigger.state {
        PathState::CalculationFailed => {
            commands.trigger(TaskEvent {
                entity: trigger.entity,
                state: TaskState::Failed,
            });
        }
        PathState::Completed => {
            commands.entity(trigger.entity).remove::<Task>();
        }
    }
    debug!("despawning observer {}", trigger.observer());
    commands.entity(trigger.observer()).despawn();
}
