use std::borrow::Cow;

use bevy::{
    ecs::{relationship::RelatedSpawnerCommands, system::IntoObserverSystem},
    prelude::*,
};

mod as_vec2;
mod neighbors;
mod ui_root;

pub use as_vec2::*;
pub use neighbors::*;
pub use ui_root::*;

pub trait AddNamedObserver {
    fn add_named_observer<E: Event, B: Bundle, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M>,
        name: impl Into<Cow<'static, str>>,
    ) -> &mut Self;
}

impl AddNamedObserver for App {
    fn add_named_observer<E: Event, B: Bundle, M>(
        &mut self,
        observer: impl IntoObserverSystem<E, B, M>,
        name: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        self.world_mut()
            .add_observer(observer)
            .insert(Name::new(name));
        self
    }
}

pub trait SpawnNamedObserver {
    fn spawn_named_observer<E: Event, B: Bundle, M>(
        &mut self,
        target: Entity,
        observer: impl IntoObserverSystem<E, B, M>,
        name: impl Into<Cow<'static, str>>,
    ) -> &mut Self;
}

impl SpawnNamedObserver for Commands<'_, '_> {
    fn spawn_named_observer<E: Event, B: Bundle, M>(
        &mut self,
        target: Entity,
        observer: impl IntoObserverSystem<E, B, M>,
        name: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        self.spawn((Observer::new(observer).with_entity(target), Name::new(name)));
        self
    }
}

impl SpawnNamedObserver for RelatedSpawnerCommands<'_, ChildOf> {
    fn spawn_named_observer<E: Event, B: Bundle, M>(
        &mut self,
        target: Entity,
        observer: impl IntoObserverSystem<E, B, M>,
        name: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        self.spawn((Observer::new(observer).with_entity(target), Name::new(name)));
        self
    }
}
