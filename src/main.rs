use bevy::{prelude::*, log::LogPlugin};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use dwarf_plugin::DwarfPlugin;
#[cfg(feature = "debug")]
use fps_plugin::FpsPlugin;
use task_plugin::TaskPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    #[cfg(feature = "debug")]
    app.add_plugin(FpsPlugin);
    app.add_plugin(TaskPlugin);
    app.add_plugin(DwarfPlugin);
    app.add_startup_system(setup);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
