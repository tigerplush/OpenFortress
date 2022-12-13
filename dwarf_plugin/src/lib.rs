use bevy::log;
use bevy::prelude::*;

pub struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::add_dwarves);
        log::info!("Loaded DwarfPlugin");
    }
}

impl DwarfPlugin {
    fn add_dwarves(mut commands: Commands) {
        commands.spawn(
                Transform::from_xyz(0.0, 0.0, 0.0)
        )
        .insert(Name::new("Dwarf"));
    }
}