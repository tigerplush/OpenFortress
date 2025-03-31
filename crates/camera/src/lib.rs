use std::ops::DerefMut;

use bevy::{math::VectorSpace, prelude::*};
use common::states::AppState;
use leafwing_input_manager::{
    plugin::InputManagerPlugin, prelude::{ActionState, InputMap, MouseScrollAxis, VirtualAxis, VirtualDPad}, Actionlike, InputManagerBundle
};

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<CameraControls>::default())
        .add_systems(OnEnter(AppState::MainGame), setup)
        .add_systems(Update, (zoom, pan.after(zoom)).run_if(in_state(AppState::MainGame)));
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum CameraControls {
    #[actionlike(DualAxis)]
    Pan,
    #[actionlike(Axis)]
    Zoom,
    #[actionlike(Axis)]
    Scroll,
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(CameraControls::Pan, VirtualDPad::wasd())
        .with_axis(CameraControls::Zoom, MouseScrollAxis::Y)
        .with_axis(CameraControls::Scroll, VirtualAxis::vertical_numpad());
    commands.spawn((InputManagerBundle::with_map(input_map), Camera2d));
}

fn zoom(query: Single<(&mut Projection, &ActionState<CameraControls>), With<Camera2d>>) {
    const CAMERA_ZOOM_RATE: f32 = 0.05;

    let (mut camera_projection, action_state) = query.into_inner();
    let zoom_delta = action_state.value(&CameraControls::Zoom);
    match camera_projection.deref_mut() {
        Projection::Orthographic(orthographic_projection) => {
            orthographic_projection.scale *= 1. - zoom_delta * CAMERA_ZOOM_RATE;
        }
        _ => unreachable!(),
    }
}

fn pan(query: Single<(&mut Transform, &ActionState<CameraControls>), With<Camera2d>>) {
    const CAMERA_PAN_RATE: f32 = 10.;

    let (mut camera_transform, action_state) = query.into_inner();

    if action_state.axis_pair(&CameraControls::Pan) != Vec2::ZERO {
        let axis_pair = action_state.axis_pair(&CameraControls::Pan);
        camera_transform.translation += (axis_pair.normalize() * CAMERA_PAN_RATE).extend(0.0);
    }
}