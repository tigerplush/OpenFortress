use bevy::prelude::*;
use std::time::Duration;

use crate::{PathEvent, PathState};
use common::types::{IWorldCoordinates, WorldCoordinates};

#[derive(Clone, Component, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Path {
    set: Vec<IWorldCoordinates>,
    current_index: usize,
    current_t: f32,
}

impl Path {
    pub(crate) fn new(set: Vec<IWorldCoordinates>) -> Self {
        Path {
            set,
            current_index: 0,
            current_t: 0.0,
        }
    }

    fn tick(&mut self, delta: Duration) {
        self.current_t += delta.as_secs_f32();
        if self.current_t > 1.0 {
            self.current_index += 1;
            self.current_t = 0.0;
        }
    }

    fn complete(&self) -> bool {
        self.current_index >= self.set.len()
    }

    fn current_position(&self) -> WorldCoordinates {
        if self.current_index + 1 >= self.set.len() {
            return self.set.last().unwrap().into();
        }
        let current = self.set[self.current_index];
        let next = self.set[self.current_index + 1];
        WorldCoordinates(current.0.as_vec3().lerp(next.0.as_vec3(), self.current_t))
    }
}

pub(crate) fn tick_path(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Path)>,
    mut commands: Commands,
) {
    for (entity, mut path) in &mut query {
        path.tick(time.delta());
        if path.complete() {
            debug!("path complete, removing path from {}", entity);
            commands.entity(entity).remove::<Path>();
            commands.trigger(PathEvent {
                entity,
                state: PathState::Completed,
            });
        }
    }
}

pub(crate) fn follow_path(mut query: Query<(&mut WorldCoordinates, &Path)>) {
    for (mut transform, path) in &mut query {
        *transform = path.current_position();
    }
}
