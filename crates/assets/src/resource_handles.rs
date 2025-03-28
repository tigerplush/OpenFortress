use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ResourceHandles {
    // Use a queue for waiting assets so they can be cycled through and moved to
    // `finished` one at a time.
    pub waiting: VecDeque<(UntypedHandle, fn(&mut World, &UntypedHandle))>,
    pub(crate) finished: Vec<UntypedHandle>,
}

impl ResourceHandles {
    /// Returns true if all requested [`Asset`]s have finished loading and are available as [`Resource`]s.
    pub fn is_all_done(&self) -> bool {
        self.waiting.is_empty()
    }
}

pub(crate) fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop_front().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push_back((handle, insert_fn));
                }
            }
        });
    });
}