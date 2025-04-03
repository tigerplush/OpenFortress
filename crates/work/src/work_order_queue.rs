use std::collections::VecDeque;

use bevy::prelude::*;

use crate::WorkOrder;

pub fn plugin(app: &mut App) {
    app.register_type::<WorkOrderQueue>()
        .insert_resource(WorkOrderQueue::new())
        .add_observer(register_work_order)
        .add_observer(unregister_work_order);
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct WorkOrderQueue(pub VecDeque<(Entity, WorkOrder)>);

impl WorkOrderQueue {
    pub fn new() -> Self {
        WorkOrderQueue(VecDeque::new())
    }
}

fn register_work_order(
    trigger: Trigger<OnAdd, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue
        .0
        .push_back((trigger.target(), *work_order));
}

fn unregister_work_order(
    trigger: Trigger<OnRemove, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue.0.retain(|(_, order)| order != work_order);
}
