use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct DwarfPlugin;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug)]
struct Position {
    pub x: i16,
    pub y: i16,
    pub elevation: i16,
}

#[derive(Component)]
struct Dwarf;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::add_dwarves);
        #[cfg(feature = "debug")]
        app.register_inspectable::<Position>();
        log::info!("Loaded DwarfPlugin");
    }
}

impl DwarfPlugin {
    fn add_dwarves(mut commands: Commands) {
        commands.spawn(
                Dwarf
        )
        .insert(Name::new("Dwarf"))
        .insert(Position {x: 0, y:0, elevation: 0});
    }
}