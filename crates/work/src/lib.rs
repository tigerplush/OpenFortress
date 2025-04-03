use bevy::prelude::*;
use common::{constants::TILE_SIZE, functions::world_to_tile};
use work_order_queue::WorkOrderQueue;

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
        );
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
            WorkOrder::Dig(pos) => TaskQueue(vec![Task::dig(pos), Task::walk_to(pos)]),
        }
    }
}

/// A task queue
#[derive(Component, Reflect)]
#[reflect(Component)]
struct TaskQueue(Vec<Task>);

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
/// A task is a concrete step taken in order to achieve some greater goal.
enum Task {
    WalkTo,
    Dig,
}

impl Task {
    fn dig(pos: &IVec3) -> Task {
        Task::Dig
    }

    fn walk_to(pos: &IVec3) -> Task {
        Task::WalkTo
    }
}

fn check_tasks(mut query: Query<(Entity, &mut TaskQueue), Without<Task>>, mut commands: Commands) {
    for (entity, mut task_queue) in &mut query {
        if let Some(task) = task_queue.0.pop() {
            info!("{} is taking on task {:?}", entity, task);
            commands.entity(entity).insert(task);
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
        if let Some((work_order_entity, _)) = work_order_queue.0.pop_front() {
            info!(
                "dwarf is taking work order for entity {}",
                work_order_entity
            );
            commands
                .entity(entity)
                .insert(CurrentWorkOrder(work_order_entity));
        }
    }
}

fn check_work_orders(
    workers: Query<(Entity, &CurrentWorkOrder)>,
    work_orders: Query<&WorkOrder>,
    mut commands: Commands,
) {
    for (entity, worker) in &workers {
        let work_order = work_orders.get(worker.0).unwrap();
        commands.entity(entity).insert(work_order.realise());
    }
}
