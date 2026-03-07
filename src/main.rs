use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::bevy_egui::{EguiGlobalSettings, EguiPlugin, PrimaryEguiContext};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Open Fortress {}", env!("CARGO_PKG_VERSION")),
            resolution: WindowResolution::new(1280, 720),
            resizable: false,
            ..default()
        }),
        ..default()
    }));
    app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
    app.add_plugins((
        animation::plugin,
        assets::plugin,
        common::plugin,
        loading_screen::plugin,
        main_game::plugin,
        menu_screen::plugin,
        splashscreen::plugin,
        ui::plugin,
        world_generation::plugin,
    ));
    app.insert_resource(EguiGlobalSettings {
        auto_create_primary_context: false,
        ..default()
    });
    app.add_observer(add_egui_context);
    app.run()
}

fn add_egui_context(trigger: On<Add, Camera2d>, mut commands: Commands) {
    commands.entity(trigger.entity).insert(PrimaryEguiContext);
}
