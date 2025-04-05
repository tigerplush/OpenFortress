use bevy::prelude::*;
use map_generation::{WorldMap, chunk_visualisation::ChunkVisualisationEvent};

use super::Task;

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Dig(pub(crate) IVec3);

pub(crate) fn handle(
    time: Res<Time>,
    mut world_map: ResMut<WorldMap>,
    query: Query<(Entity, &Dig)>,
    mut commands: Commands,
) {
    for (entity, dig) in &query {
        if world_map.damage_block(dig.0, time.delta_secs()) {
            commands.entity(entity).remove::<Dig>().remove::<Task>();
            commands.trigger(ChunkVisualisationEvent::SetDirty(dig.0));
        }
    }
}
