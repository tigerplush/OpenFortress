use bevy::prelude::*;
use walk_to::WalkTo;

pub mod walk_to;

/// A task queue
#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct TaskQueue(Vec<Task>);

impl TaskQueue {
    pub(crate) fn new(tasks: &[Task]) -> Self {
        TaskQueue(tasks.to_vec())
    }
}

#[derive(Clone, Component, Debug, Reflect)]
#[reflect(Component)]
/// A task is a concrete step taken in order to achieve some greater goal.
pub(crate) enum Task {
    WalkTo(WalkTo),
    Dig,
}

impl Task {
    pub(crate) fn dig(pos: &IVec3) -> Task {
        Task::Dig
    }

    pub(crate) fn walk_to(pos: &IVec3) -> Task {
        Task::WalkTo(WalkTo(*pos))
    }
}

pub(crate) fn check_tasks(
    mut query: Query<(Entity, &mut TaskQueue), Without<Task>>,
    mut commands: Commands,
) {
    for (entity, mut task_queue) in &mut query {
        if let Some(task) = task_queue.0.pop() {
            info!("{} is taking on task {:?}", entity, task);
            match task {
                Task::WalkTo(walk_to) => {
                    commands.entity(entity).insert(walk_to);
                }
                Task::Dig => (),
            };
            commands.entity(entity).insert(task);
        }
    }
}
