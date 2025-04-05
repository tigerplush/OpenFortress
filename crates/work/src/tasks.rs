use bevy::prelude::*;
use dig::Dig;
use map_generation::WorldMap;
use walk_to::WalkTo;

pub mod dig;
pub mod walk_to;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<TaskQueue>()
        .register_type::<Task>()
        .register_type::<Dig>()
        .register_type::<WalkTo>()
        .add_systems(
            Update,
            (
                check_tasks,
                dig::handle.run_if(resource_exists::<WorldMap>),
                walk_to::handle,
            ),
        );
}

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
    Dig(Dig),
}

impl Task {
    pub(crate) fn dig(pos: &IVec3) -> Task {
        Task::Dig(Dig(*pos))
    }

    pub(crate) fn walk_to(pos: &IVec3) -> Task {
        Task::WalkTo(WalkTo(*pos))
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
