use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Open Fortress {}", env!("CARGO_PKG_VERSION")),
            resolution: WindowResolution::new(1280.0, 720.0),
            resizable: false,
            ..default()
        }),
        ..default()
    }));
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_plugins((
        animation::plugin,
        assets::plugin,
        common::plugin,
        loading_screen::plugin,
        main_game::plugin,
        menu_screen::plugin,
        splashscreen::plugin,
        ui::plugin,
    ));
    app.run();
}
