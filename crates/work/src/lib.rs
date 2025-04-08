use bevy::prelude::*;
use common::{
    constants::TILE_SIZE,
    functions::world_position_to_world_coordinates,
    traits::{AddNamedObserver, SpawnNamedObserver},
    types::WorldCoordinates,
};
use tasks::{Task, TaskEvent, TaskQueue};
use work_order_queue::WorkOrderQueue;

mod tasks;
pub mod work_order_queue;

pub fn plugin(app: &mut App) {
    app.register_type::<WorkOrder>()
        .register_type::<CurrentWorkOrder>()
        .add_plugins((tasks::plugin, work_order_queue::plugin))
        .add_systems(Update, (fetch_new_work_order, check_work_orders));
}

#[derive(Clone, Component, Copy, PartialEq, Reflect)]
pub enum WorkOrder {
    Dig(WorldCoordinates),
}

impl WorkOrder {
    pub fn dig(world_position: Vec3) -> impl Bundle {
        let world_coordinates = world_position_to_world_coordinates(world_position);
        (
            Name::new(format!("WorkOrder - Dig {}", world_coordinates.0)),
            Transform::from_translation(
                (world_position / TILE_SIZE.extend(1.0)).round() * TILE_SIZE.extend(1.0),
            ),
            WorkOrder::Dig(world_coordinates),
        )
    }

    pub fn realise(&self) -> impl Bundle {
        match self {
            WorkOrder::Dig(pos) => TaskQueue::new(&[Task::dig(*pos), Task::walk_to(*pos)]),
        }
    }
}

/// Marks an entity as a worker, i.e. someone who can fulfill work orders
#[derive(Component)]
pub struct Worker;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrentWorkOrder(Entity);

fn fetch_new_work_order(
    mut work_order_queue: ResMut<WorkOrderQueue>,
    query: Query<Entity, (With<Worker>, Without<CurrentWorkOrder>)>,
    mut commands: Commands,
) {
    for worker_entity in &query {
        if let Some((work_order_entity, work_order)) = work_order_queue.pending.pop_front() {
            info!(
                "dwarf is taking work order for entity {}",
                work_order_entity
            );
            work_order_queue
                .in_progress
                .push_back((work_order_entity, work_order));
            let target = commands
                .entity(worker_entity)
                .insert(CurrentWorkOrder(work_order_entity))
                .id();
            commands.spawn_named_observer(target, on_task_finished, "on_task_finished");
        }
    }
}

fn check_work_orders(
    workers: Query<(Entity, &CurrentWorkOrder), Without<TaskQueue>>,
    work_orders: Query<&WorkOrder>,
    mut commands: Commands,
) {
    for (entity, worker) in &workers {
        let work_order = work_orders.get(worker.0).unwrap();
        commands.entity(entity).insert(work_order.realise());
    }
}

fn on_task_finished(
    trigger: Trigger<TaskEvent>,
    workers: Query<&CurrentWorkOrder>,
    mut commands: Commands,
) {
    match trigger.event() {
        TaskEvent::Completed => {
            // on task completed:
            // remove CurrentWorkOrder from worker
            commands
                .entity(trigger.target())
                .remove::<CurrentWorkOrder>();
            // despawn WorkOrder
            if let Ok(current_work_order) = workers.get(trigger.target()) {
                commands.entity(current_work_order.0).despawn();
            }
            // despawn the observer
            commands.entity(trigger.observer()).despawn();
        }
    }
}
