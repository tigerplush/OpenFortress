use bevy::prelude::*;

use crate::components::work_order::WorkOrder;

pub fn plugin(app: &mut App) {
    app
    .register_type::<WorkOrderQueue>()
    .add_observer(register_work_order)
    .add_observer(unregister_work_order);
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct WorkOrderQueue(pub Vec<WorkOrder>);

impl WorkOrderQueue {
    pub fn new() -> Self {
        WorkOrderQueue(Vec::new())
    }
}

fn register_work_order(
    trigger: Trigger<OnAdd, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue.0.push(*work_order);
}

fn unregister_work_order(
    trigger: Trigger<OnRemove, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue.0.retain(|order| order != work_order);
}