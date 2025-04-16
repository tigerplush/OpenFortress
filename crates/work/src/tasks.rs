use bevy::prelude::*;
use common::types::WorldCoordinates;
use dig::Dig;
use map_generation::map_generation::WorldMap;
use walk_to::WalkTo;
use walk_to_nearest::WalkToNearest;

pub mod dig;
pub mod walk_to;
pub mod walk_to_nearest;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<TaskQueue>()
        .register_type::<Task>()
        .register_type::<Dig>()
        .register_type::<WalkToNearest>()
        .register_type::<WalkTo>()
        .add_systems(
            Update,
            (
                check_tasks,
                dig::handle.run_if(resource_exists::<WorldMap>),
                walk_to_nearest::handle,
                walk_to::handle,
            ),
        );
}

/// A queue of tasks that a worker will try to fulfill
///
/// These are in reverse order and will be popped of the stack.
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
    WalkToNearest(WalkToNearest),
    WalkTo(WalkTo),
    Dig(Dig),
}

impl Task {
    pub(crate) fn dig(pos: WorldCoordinates) -> Task {
        Task::Dig(Dig(pos))
    }

    pub(crate) fn walk_to(pos: WorldCoordinates) -> Task {
        Task::WalkTo(WalkTo(pos))
    }

    pub(crate) fn walk_to_nearest(pos: WorldCoordinates) -> Task {
        Task::WalkToNearest(WalkToNearest(pos))
    }
}

#[derive(Event)]
pub enum TaskEvent {
    Completed,
}

pub(crate) fn check_tasks(
    mut query: Query<(Entity, &mut TaskQueue), Without<Task>>,
    mut commands: Commands,
) {
    for (entity, mut task_queue) in &mut query {
        if let Some(task) = task_queue.0.pop() {
            info!("{} is taking on task {:?}", entity, task);
            match task {
                Task::WalkToNearest(walk_to_nearest) => {
                    commands.entity(entity).insert(walk_to_nearest);
                }
                Task::WalkTo(walk_to) => {
                    commands.entity(entity).insert(walk_to);
                }
                Task::Dig(dig) => {
                    commands.entity(entity).insert(dig);
                }
            };
            commands.entity(entity).insert(task);
        } else {
            commands
                .entity(entity)
                .remove::<TaskQueue>()
                .trigger(TaskEvent::Completed);
        }
    }
}
