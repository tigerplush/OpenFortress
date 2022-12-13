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

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug)]
struct Desire {
    pub name: Name,
    pub value: f32,
    pub increase: f32,
}

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::add_dwarves);
        app.add_system(Self::tick_desires);
        #[cfg(feature = "debug")]
        app.register_inspectable::<Position>();
        #[cfg(feature = "debug")]
        app.register_inspectable::<Desire>();
        log::info!("Loaded DwarfPlugin");
    }
}

impl DwarfPlugin {
    fn add_dwarves(mut commands: Commands) {
        commands.spawn(
                Dwarf
        )
        .insert(Name::new("Dwarf"))
        .insert(Position {x: 0, y:0, elevation: 0})
        .insert(Desire { name: Name::new("Hunger"), value: 0.0, increase: 0.1});
    }

    fn tick_desires(
        time: Res<Time>,
        mut query:Query<&mut Desire>
    ) {
        for mut desire in query.iter_mut() {
            desire.value = desire.value + desire.increase * time.delta_seconds();
        }
    }
}