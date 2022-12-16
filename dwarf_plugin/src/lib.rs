
use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

use data::*;

pub struct DwarfPlugin;

#[derive(Component)]
struct Dwarf;

#[derive(Component)]
struct Food;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug)]
struct Desire {
    pub value: f32,
    pub increase: f32,
    pub threshold: f32,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
enum DesireType {
    Hunger,
    Socialize,
}

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::add_dwarves);
        app.add_startup_system(Self::spawn_food);
        app.add_system(Self::tick_desires);
        app.add_system(Self::calc_dist);
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Position>();
            app.register_inspectable::<Desire>();
            app.register_inspectable::<DesireType>();
        }
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
        .with_children(|parent| {
            parent.spawn(Desire { value: 0.0, increase: 0.0, threshold: 70.0})
            .insert(DesireType::Hunger)
            .insert(Name::new("Hunger"));
        })
        .with_children(|parent| {
            parent.spawn(Desire { value: 0.0, increase: 0.0, threshold: 70.0})
            .insert(DesireType::Socialize)
            .insert(Name::new("Socialize"));
        });
    }

    fn spawn_food(mut commands: Commands) {
        commands.spawn(Food)
        .insert(Name::new("Food"))
        .insert(Position::random());
    }

    fn tick_desires(
        time: Res<Time>,
        mut query:Query<(&mut Desire, &DesireType, &Parent)>
    ) {
        for (mut desire, _desire_type, _parent) in query.iter_mut() {
            desire.value = desire.value + desire.increase * time.delta_seconds();
        }
    }

    fn calc_dist(
        mut commands: Commands,
        dwarf_query: Query<&Position, With<Dwarf>>,
        food_query: Query<(&Position, Entity), With<Food>>
    )
    {
        if let Some(d) = dwarf_query.iter().next() {
            if let Some(f) = food_query.iter().next() {
                if Position::calculate_path(*d, *f.0) {
                    commands.entity(f.1).despawn();
                }
            }
        } 
    }
}