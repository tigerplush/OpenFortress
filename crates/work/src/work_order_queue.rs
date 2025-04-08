use std::collections::VecDeque;

use bevy::prelude::*;
use common::traits::AddNamedObserver;

use crate::WorkOrder;

pub fn plugin(app: &mut App) {
    app.register_type::<WorkOrderQueue>()
        .insert_resource(WorkOrderQueue::default())
        .add_named_observer(register_work_order, "register_work_order")
        .add_named_observer(unregister_work_order, "unregister_work_order");
}

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct WorkOrderQueue {
    pub(crate) pending: VecDeque<(Entity, WorkOrder)>,
    pub(crate) in_progress: VecDeque<(Entity, WorkOrder)>,
}

impl WorkOrderQueue {
    pub fn contains(&self, item: &WorkOrder) -> bool {
        self.pending
            .iter()
            .any(|(_, work_order)| work_order == item)
            || self
                .in_progress
                .iter()
                .any(|(_, work_order)| work_order == item)
    }
}

fn register_work_order(
    trigger: Trigger<OnAdd, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue
        .pending
        .push_back((trigger.target(), *work_order));
}

fn unregister_work_order(
    trigger: Trigger<OnRemove, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue
        .pending
        .retain(|(_, order)| order != work_order);
    work_order_queue
        .in_progress
        .retain(|(_, order)| order != work_order);
}
