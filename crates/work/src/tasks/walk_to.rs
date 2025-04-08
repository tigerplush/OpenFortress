use bevy::prelude::*;
use common::{
    functions::world_position_to_world_coordinates,
    traits::{AddNamedObserver, SpawnNamedObserver},
    types::WorldCoordinates,
};
use pathfinding::{Pathfinder, path::Path};

use super::Task;

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct WalkTo(pub(crate) WorldCoordinates);

pub(crate) fn handle(query: Query<(Entity, &Transform, &WalkTo)>, mut commands: Commands) {
    for (entity, transform, walk_to) in &query {
        info!("inserting pathfinding component");
        let start = world_position_to_world_coordinates(transform.translation);
        let target = commands
            .entity(entity)
            .remove::<WalkTo>()
            .insert(Pathfinder::new(start, walk_to.0))
            .id();
        commands.spawn_named_observer(target, on_path_completed, "on_path_complete");
    }
}

fn on_path_completed(trigger: Trigger<OnRemove, Path>, mut commands: Commands) {
    commands.entity(trigger.target()).remove::<Task>();
    commands.entity(trigger.observer()).despawn();
}
