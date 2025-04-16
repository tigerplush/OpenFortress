use bevy::prelude::*;
use common::{
    functions::world_position_to_world_coordinates, traits::SpawnNamedObserver,
    types::WorldCoordinates,
};
use pathfinding::{PathEvent, pathfinder::Pathfinder};

use super::{Task, TaskEvent};

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct WalkToNearest(pub(crate) WorldCoordinates);

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

fn on_path_event(trigger: Trigger<PathEvent>, mut commands: Commands) {
    match trigger.event() {
        PathEvent::CalculationFailed => {
            commands.entity(trigger.target()).trigger(TaskEvent::Failed);
        }
        PathEvent::Completed => {
            commands.entity(trigger.target()).remove::<Task>();
        }
    }
    commands.entity(trigger.observer()).despawn();
}
