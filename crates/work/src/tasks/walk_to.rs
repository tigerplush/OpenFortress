use bevy::prelude::*;
use common::{
    traits::SpawnNamedObserver,
    types::{BlockCoordinates, WorldCoordinates},
};
use pathfinding::{path::Path, pathfinder::Pathfinder};

use super::Task;

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct WalkTo(pub(crate) BlockCoordinates);

pub(crate) fn handle(query: Query<(Entity, &WorldCoordinates, &WalkTo)>, mut commands: Commands) {
    for (entity, coordinates, walk_to) in &query {
        info!("inserting pathfinding component");
        let start = coordinates.block();
        let target = commands
            .entity(entity)
            .remove::<WalkTo>()
            .insert(Pathfinder::exact(start, walk_to.0))
            .id();
        commands.spawn_named_observer(target, on_path_completed, "on_path_complete");
    }
}

fn on_path_completed(trigger: On<Remove, Path>, mut commands: Commands) {
    commands.entity(trigger.entity).remove::<Task>();
    debug!("despawning observer {}", trigger.observer());
    commands.entity(trigger.observer()).despawn();
}
