use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::bevy_egui::EguiContexts;
use camera::CameraLayer;
use common::{
    functions::world_position_to_world_coordinates, states::AppState, types::WorldCoordinates,
};
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap, MouseMove},
};
use map_generation::map_generation::WorldMap;
use work::{WorkOrder, work_order_queue::WorkOrderQueue};

use crate::ui;

#[derive(Default, PartialEq, Reflect)]
pub(crate) enum MouseActions {
    #[default]
    None,
    Dig,
}

#[derive(Event)]
enum BrushInputEvent {
    Designated(WorldCoordinates),
}

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub(crate) struct BrushSettings {
    pub(crate) current_action: MouseActions,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(BrushSettings::default())
        .add_plugins(InputManagerPlugin::<MouseControls>::default())
        .add_event::<BrushInputEvent>()
        .add_systems(OnEnter(AppState::MainGame), setup_brush)
        .add_systems(
            Update,
            (
                handle_brush_input,
                handle_brush.after(handle_brush_input),
                ui::brushes,
            )
                .run_if(in_state(AppState::MainGame)),
        );
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
    commands.spawn((Name::new("Brush Controls"), input_map));
}

fn handle_brush_input(
    query: Single<&ActionState<MouseControls>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform, &CameraLayer), With<Camera>>,
    mut brush_event_writer: EventWriter<BrushInputEvent>,
    mut contexts: EguiContexts,
) {
    // Skip pointer events that are captured by egui
    let ctx = contexts.ctx_mut();
    if ctx.wants_pointer_input() {
        return;
    }

    let window = window.into_inner();
    let action_state = query.into_inner();
    let (camera, camera_transform, layer) = camera.into_inner();
    if action_state.pressed(&MouseControls::PrimaryAction) {
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let world_coordinates =
                world_position_to_world_coordinates(world_position.extend(layer.0 as f32));
            brush_event_writer.write(BrushInputEvent::Designated(world_coordinates));
        }
    }
}

fn handle_brush(
    brush_settings: Res<BrushSettings>,
    world_map: Res<WorldMap>,
    work_order_queue: Res<WorkOrderQueue>,
    mut brush_event_reader: EventReader<BrushInputEvent>,
    mut commands: Commands,
) {
    for brush_input_event in brush_event_reader.read() {
        #[allow(irrefutable_let_patterns)]
        if let BrushInputEvent::Designated(world_coordinate) = brush_input_event {
            match brush_settings.current_action {
                MouseActions::Dig => {
                    if !work_order_queue.contains(&WorkOrder::Dig(*world_coordinate))
                        && world_map.get_block(*world_coordinate).is_some()
                    {
                        commands.spawn(WorkOrder::dig(*world_coordinate));
                    }
                }
                MouseActions::None => (),
            }
        }
    }
}
