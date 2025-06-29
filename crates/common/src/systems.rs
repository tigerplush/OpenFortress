use crate::{constants::TILE_SIZE, types::WorldCoordinates};
use bevy::prelude::*;

pub fn apply_world_coordinates(
    mut query: Query<(&mut Transform, &WorldCoordinates), Changed<WorldCoordinates>>,
) {
    for (mut transform, coordinates) in query.iter_mut() {
        transform.translation.x = coordinates.0.x * TILE_SIZE.x;
        transform.translation.y = coordinates.0.y * TILE_SIZE.y;
        transform.translation.z = 0.1;
        transform.set_changed();
    }
}
