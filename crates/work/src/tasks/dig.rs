use bevy::prelude::*;
use common::types::IWorldCoordinates;
use map_generation::messages::{BlockUpdate, UpdateMap};

use crate::tasks::Task;

#[derive(Clone, Component, Copy, Debug, Reflect)]
#[reflect(Component)]
#[require(DigTimer)]
pub(crate) struct Dig(pub(crate) IWorldCoordinates);

#[derive(Component, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub(crate) struct DigTimer(Timer);

impl Default for DigTimer {
    fn default() -> Self {
        DigTimer(Timer::from_seconds(0.25, TimerMode::Repeating))
    }
}

pub(crate) fn tick(
    time: Res<Time>,
    mut query: Query<(&Dig, &mut DigTimer)>,
    mut message_writer: MessageWriter<UpdateMap>,
) {
    for (dig, mut dig_timer) in &mut query {
        dig_timer.tick(time.delta());
        if dig_timer.just_finished() {
            message_writer.write(UpdateMap::DamageBlock(dig.0, 0.25));
            debug!("Hurting block {:?}", dig.0);
        }
    }
}

pub(crate) fn cleanup(
    mut message_reader: MessageReader<BlockUpdate>,
    query: Query<(Entity, &Dig)>,
    mut commands: Commands,
) {
    for block_update in message_reader.read() {
        if let BlockUpdate::Removed(coordinates) = block_update {
            for (entity, dig) in &query {
                debug!(
                    "Block {:?} is destroyed, removing task from {}",
                    coordinates, entity
                );
                if dig.0 == *coordinates {
                    commands
                        .entity(entity)
                        .remove::<Dig>()
                        .remove::<DigTimer>()
                        .remove::<Task>();
                }
            }
        }
    }
}
