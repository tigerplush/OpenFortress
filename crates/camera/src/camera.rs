use std::ops::DerefMut;

use bevy::prelude::*;
use common::states::AppState;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap, MouseScrollAxis, VirtualDPad},
};

#[derive(Default)]
pub struct CameraPlugin(CameraSettings);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CameraLayer>()
            .register_type::<CameraSettings>()
            .insert_resource(self.0)
            .add_plugins(InputManagerPlugin::<CameraControls>::default())
            .add_systems(OnEnter(AppState::MainGame), setup)
            .add_systems(
                Update,
                (zoom, scroll, pan.after(zoom)).run_if(in_state(AppState::MainGame)),
            );
    }
}

#[derive(Clone, Copy, Reflect, Resource)]
#[reflect(Resource)]
struct CameraSettings {
    zoom_rate: f32,
    pan_rate: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            zoom_rate: 0.05,
            pan_rate: 10.,
        }
    }
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum CameraControls {
    #[actionlike(DualAxis)]
    Pan,
    #[actionlike(Axis)]
    Zoom,
    ScrollUp,
    ScrollDown,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraLayer(pub i32);

fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(CameraControls::Pan, VirtualDPad::wasd())
        .with_axis(CameraControls::Zoom, MouseScrollAxis::Y)
        .with(CameraControls::ScrollUp, KeyCode::Numpad8)
        .with(CameraControls::ScrollDown, KeyCode::Numpad2);
    commands.spawn((
        input_map,
        Camera2d,
        CameraLayer(0),
        StateScoped(AppState::MainGame),
    ));
}

fn zoom(
    camera_settings: Res<CameraSettings>,
    query: Single<(&mut Projection, &ActionState<CameraControls>), With<Camera2d>>,
) {
    let (mut camera_projection, action_state) = query.into_inner();
    let zoom_delta = action_state.value(&CameraControls::Zoom);
    match camera_projection.deref_mut() {
        Projection::Orthographic(orthographic_projection) => {
            orthographic_projection.scale *= 1. - zoom_delta * camera_settings.zoom_rate;
        }
        _ => unreachable!(),
    }
}

fn scroll(query: Single<(&mut CameraLayer, &ActionState<CameraControls>), With<Camera2d>>) {
    let (mut layer, action_state) = query.into_inner();
    let mut delta = 0;
    if action_state.just_pressed(&CameraControls::ScrollUp) {
        delta += 1;
    }
    if action_state.just_pressed(&CameraControls::ScrollDown) {
        delta -= 1;
    }
    layer.0 += delta;
}

fn pan(
    camera_settings: Res<CameraSettings>,
    query: Single<(&mut Transform, &ActionState<CameraControls>), With<Camera2d>>,
) {
    let (mut camera_transform, action_state) = query.into_inner();

    if action_state.axis_pair(&CameraControls::Pan) != Vec2::ZERO {
        let axis_pair = action_state.axis_pair(&CameraControls::Pan);
        camera_transform.translation +=
            (axis_pair.normalize() * camera_settings.pan_rate).extend(0.0);
    }
}
