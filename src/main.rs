use bevy::prelude::*;
use dwarf_plugin::DwarfPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DwarfPlugin)
        .run();
}
