use bevy::prelude::*;
use common::{constants::TILE_SIZE, functions::world_to_tile};
use tasks::{Task, TaskQueue, check_tasks};
use work_order_queue::WorkOrderQueue;

mod tasks;
pub mod work_order_queue;

pub fn plugin(app: &mut App) {
    app.register_type::<WorkOrder>()
        .register_type::<TaskQueue>()
        .register_type::<Task>()
        .register_type::<CurrentWorkOrder>()
        .add_plugins(work_order_queue::plugin)
        .add_systems(
            Update,
            (fetch_new_work_order, check_work_orders, check_tasks),
        )
        .add_systems(Update, tasks::walk_to::handle_walk_to);
}

#[derive(Clone, Component, Copy, PartialEq, Reflect)]
pub enum WorkOrder {
    Dig(IVec3),
}

impl WorkOrder {
    pub fn dig(world_position: Vec3) -> impl Bundle {
        let tile_coordinates = world_to_tile(world_position);
        (
            Name::new(format!("WorkOrder - Dig {}", tile_coordinates)),
            Transform::from_translation(
                (world_position / TILE_SIZE.extend(1.0)).round() * TILE_SIZE.extend(1.0),
            ),
            WorkOrder::Dig(tile_coordinates),
        )
    }

    pub fn realise(&self) -> impl Bundle {
        match self {
            WorkOrder::Dig(pos) => TaskQueue::new(&[Task::dig(pos), Task::walk_to(pos)]),
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
    for entity in &query {
        if let Some((work_order_entity, work_order)) = work_order_queue.pending.pop_front() {
            info!(
                "dwarf is taking work order for entity {}",
                work_order_entity
            );
            work_order_queue
                .in_progress
                .push_back((work_order_entity, work_order));
            commands
                .entity(entity)
                .insert(CurrentWorkOrder(work_order_entity));
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
