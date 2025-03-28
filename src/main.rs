use bevy::prelude::*;
use bevy::window::WindowResolution;

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
    app.add_plugins((
        assets::plugin,
        common::plugin,
        loading_screen::plugin,
        menu_screen::plugin,
        splashscreen::plugin,
        ui::plugin,
    ));
    app.run();
}
