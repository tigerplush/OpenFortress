use assets::{icon_asset::IconAsset, tileset_asset::TILE_SIZE};
use bevy::{prelude::*, window::PrimaryWindow};
use camera::CameraLayer;
use common::states::AppState;
use dwarf::Dwarf;
use leafwing_input_manager::{
    Actionlike, InputManagerBundle,
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap, MouseMove},
};
use map_generation::{WorldMap, world_to_tile};

pub fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, dwarf::plugin, map_generation::plugin))
        .add_systems(OnEnter(AppState::MainGame), setup)
        .register_type::<WorkOrderQueue>()
        .add_plugins(InputManagerPlugin::<MouseControls>::default())
        .add_systems(OnEnter(AppState::MainGame), setup_brush)
        .add_systems(Update, (handle_brush,).run_if(in_state(AppState::MainGame)))
        .add_observer(register_work_order)
        .add_observer(unregister_work_order)
        .add_observer(add_vis_to_work_order);
}

fn setup(mut commands: Commands) {
    // for _ in 0..7 {
    commands.spawn(Dwarf);
    // }
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum MouseControls {
    #[actionlike(DualAxis)]
    Move,
    PrimaryAction,
}

fn setup_brush(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(MouseControls::Move, MouseMove::default())
        .with(MouseControls::PrimaryAction, MouseButton::Left);
    commands.spawn((
        Name::new("Brush Controls"),
        InputManagerBundle::with_map(input_map),
    ));

    commands.insert_resource(WorkOrderQueue::new());
}

fn handle_brush(
    world_map: Res<WorldMap>,
    work_order_queue: Res<WorkOrderQueue>,
    query: Single<&ActionState<MouseControls>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform, &CameraLayer), With<Camera>>,
    mut commands: Commands,
) {
    let window = window.into_inner();
    let action_state = query.into_inner();
    let (camera, camera_transform, layer) = camera.into_inner();
    if action_state.pressed(&MouseControls::PrimaryAction) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let tile_coordinates = world_to_tile(world_position.extend(layer.0 as f32));
            if !work_order_queue.0.contains(&WorkOrder::Dig(tile_coordinates))
                && world_map.get_tile(tile_coordinates).is_some()
            {
                commands.spawn(WorkOrder::dig(world_position.extend(layer.0 as f32)));
            }
        }
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct WorkOrderQueue(Vec<WorkOrder>);

impl WorkOrderQueue {
    fn new() -> Self {
        WorkOrderQueue(Vec::new())
    }
}

#[derive(Clone, Component, Copy, PartialEq, Reflect)]
enum WorkOrder {
    Dig(IVec3),
}

impl WorkOrder {
    fn dig(world_position: Vec3) -> impl Bundle {
        let tile_coordinates = world_to_tile(world_position);
        (
            Name::new(format!("WorkOrder - Dig {}", tile_coordinates)),
            Transform::from_translation((world_position / TILE_SIZE.extend(1.0)).round() * TILE_SIZE.extend(1.0)),
            WorkOrder::Dig(tile_coordinates),
        )
    }
}

fn register_work_order(
    trigger: Trigger<OnAdd, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue.0.push(*work_order);
}

fn unregister_work_order(
    trigger: Trigger<OnRemove, WorkOrder>,
    mut work_order_queue: ResMut<WorkOrderQueue>,
    work_orders: Query<&WorkOrder>,
) {
    let work_order = work_orders.get(trigger.target()).unwrap();
    work_order_queue.0.retain(|order| order != work_order);
}

fn add_vis_to_work_order(
    trigger: Trigger<OnAdd, WorkOrder>,
    icon_asset: Res<IconAsset>,
    mut commands: Commands,
){
    commands.entity(trigger.target()).insert(icon_asset.sprite(IconAsset::SHOVEL));
}