use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::log;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;


pub struct TaskPlugin;

impl Plugin for TaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(execute_task);
        app.add_system(spawn_next_task);
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<TaskQueue>();
            app.register_inspectable::<Task>();
        }
        log::info!("Loaded TaskPlugin");
    }
}

fn execute_task(
    mut query: Query<&mut Task>
) {
    for mut task in query.iter_mut() {
        task.execute();
    }
}

fn spawn_next_task(
    mut commands: Commands,
    mut query: Query<(&mut TaskQueue, Entity), Without<Task>>
) {
    for (mut queue, entity) in query.iter_mut() {
        if let Some(task) = queue.next_task() {
            commands
                .entity(entity)
                .insert(task);
        }
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Default)]
pub struct Task {
    name: Name,
}

impl Task {
    pub fn new(name: Name) -> Self {
        Task {
            name: name
        }
    }
    fn execute(&mut self) {

    }
}

#[derive(Clone, Copy)]
enum State {
    FindFood,
    CalculatePath,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
pub struct TaskQueue {
    queue: VecDeque<Task>
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            queue: VecDeque::new()
        }
    }

    pub fn push_back(
        &mut self,
        task: Task
    ) {
        self.queue.push_back(task);
    }

    fn next_task(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }
}