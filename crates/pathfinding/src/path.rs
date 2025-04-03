use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Path {
    set: Vec<Vec3>,
    current_index: usize,
    current_t: f32,
}

impl Path {
    pub(crate) fn new(set: Vec<Vec3>) -> Self {
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

    fn current_position(&self) -> Vec3 {
        if self.current_index + 1 >= self.set.len() {
            return *self.set.last().unwrap();
        }
        let current = self.set[self.current_index];
        let next = self.set[self.current_index + 1];
        current.lerp(next, self.current_t)
    }
}

pub(crate) fn tick_path(time: Res<Time>, mut query: Query<&mut Path>) {
    for mut path in &mut query {
        path.tick(time.delta());
    }
}

pub(crate) fn follow_path(mut query: Query<(&mut Transform, &Path)>) {
    for (mut transform, path) in &mut query {
        transform.translation = path.current_position();
    }
}
