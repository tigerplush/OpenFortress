use bevy::prelude::*;
use common::types::IWorldCoordinates;

/// Messages to tell the map to update specific contents.
#[derive(Message)]
pub enum UpdateMap {
    /// Tells the map to damage a block with the given coordinates and the
    /// given damage
    DamageBlock(IWorldCoordinates, f32),
    ScheduleForRemoval(IWorldCoordinates),
}

#[derive(Message)]
pub enum BlockUpdate {
    Added,
    Removed(IWorldCoordinates),
    ScheduleForRemoval(IWorldCoordinates),
}
