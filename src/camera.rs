use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(CameraSettings {
        zoom_speed: 5.0,
        pan_speed: 500.0,
    })
    .add_plugins(InputManagerPlugin::<CameraControls>::default())
    .add_systems(Startup, setup)
    .add_systems(Update, update);
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_axis(CameraControls::Zoom, MouseScrollAxis::Y)
        .with_dual_axis(CameraControls::Pan, KeyboardVirtualDPad::WASD)
        .with_dual_axis(CameraControls::Pan, KeyboardVirtualDPad::ARROW_KEYS);

    // others from the discord server don't recommend to move the 2d camera away from z:999.9
    // when it becomes an issue, change it
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 500.0),
            ..default()
        },
        InputManagerBundle::with_map(input_map),
    ));
}

#[derive(Resource)]
struct CameraSettings {
    zoom_speed: f32,
    pan_speed: f32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum CameraControls {
    Zoom,
    Pan,
}

impl Actionlike for CameraControls {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            CameraControls::Zoom => InputControlKind::Axis,
            CameraControls::Pan => InputControlKind::DualAxis,
        }
    }
}

fn update(
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut query: Query<
        (
            &ActionState<CameraControls>,
            &mut Transform,
            &mut OrthographicProjection,
        ),
        With<Camera>,
    >,
) {
    let Ok((action, mut transform, mut projection)) = query.get_single_mut() else {
        return;
    };

    let axis_pair = action.axis_pair(&CameraControls::Pan);
    transform.translation += Vec3::new(axis_pair.x, axis_pair.y, 0.0).normalize_or_zero()
        * time.delta_seconds()
        * settings.pan_speed;

    let zoom_delta = action.value(&CameraControls::Zoom);
    projection.scale *= 1.0 - zoom_delta * settings.zoom_speed * time.delta_seconds();
}
