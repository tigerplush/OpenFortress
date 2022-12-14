use queues::*;

use bevy::log;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use rand::{{thread_rng, Rng}};
pub struct DwarfPlugin;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug)]
struct Position {
    pub x: i16,
    pub y: i16,
    pub elevation: i16,
}

impl Position {
    fn random() -> Self {
        let mut rng = thread_rng();
        Position {x: rng.gen::<i16>(), y: rng.gen::<i16>(), elevation: rng.gen::<i16>() }
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Component, Default)]
struct Task {

}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component)]
struct TaskQueue {
    #[cfg_attr(feature = "debug", inspectable(ignore))]
    queue: Queue<Task>,
    //list: Vec<Task>
}

impl TaskQueue {
    fn new() -> Self {
        TaskQueue { queue: Queue::new()}
    }

    fn push(&self) {
        log::info!("Pushed something");
    }

    fn contains(&self, desire_type: &DesireType) -> bool {
        true
    }
}

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
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Position>();
            app.register_inspectable::<Desire>();
            app.register_inspectable::<DesireType>();
            app.register_inspectable::<TaskQueue>();
            app.register_inspectable::<Task>();
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
            parent.spawn(Desire { value: 0.0, increase: 0.1, threshold: 70.0})
            .insert(DesireType::Hunger)
            .insert(Name::new("Hunger"));
        })
        .with_children(|parent| {
            parent.spawn(Desire { value: 0.0, increase: 0.1, threshold: 70.0})
            .insert(DesireType::Socialize)
            .insert(Name::new("Socialize"));
        })
        .insert(TaskQueue::new());
    }

    fn spawn_food(mut commands: Commands) {
        commands.spawn(Food)
        .insert(Name::new("Food"))
        .insert(Position::random());
    }

    fn tick_desires(
        time: Res<Time>,
        mut query:Query<(&mut Desire, &DesireType, &Parent)>,
        parent_query: Query<&mut TaskQueue>
    ) {
        for (mut desire, desire_type, parent) in query.iter_mut() {
            desire.value = desire.value + desire.increase * time.delta_seconds();
            let queue = match parent_query.get(parent.get()) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    continue;
                }
            };
            if desire.value > desire.threshold && !queue.contains(desire_type){
                queue.push();
            }
        }
    }
}