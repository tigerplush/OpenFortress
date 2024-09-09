use bevy::{color::palettes::tailwind::GRAY_50, math::VectorSpace, prelude::*, window::PrimaryWindow};
use leafwing_input_manager::prelude::*;

use crate::{
    map::MapController,
    Position,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<ToolControls>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update);
    app.add_systems(Update, draw_gizmos);
}

#[derive(Actionlike, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
enum ToolControls {
    Confirm,
    Cancel,
}

#[derive(Component)]
struct StartPosition(Position);
#[derive(Component)]
struct EndPosition(Position);

fn setup(mut commands: Commands) {
    let input_map = InputMap::default()
        .with(ToolControls::Confirm, MouseButton::Left)
        .with(ToolControls::Cancel, MouseButton::Right);
    commands.spawn((InputManagerBundle::with_map(input_map),));
}

fn update(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    control_query: Query<(Entity, &ActionState<ToolControls>)>,
    map_query: Query<&MapController>,
    mut commands: Commands,
) {
    let Ok(map) = map_query.get_single() else {
        return;
    };
    let Ok((entity, action)) = control_query.get_single() else {
        return;
    };

    let Ok((camera, transform)) = camera_query.get_single() else {
        return;
    };

    if action.just_pressed(&ToolControls::Confirm) {
        if let Some(world_position) = get_pos(window_query.single(), camera, transform) {
            let pos = Position::from_world(world_position.extend(map.current_level as f32));
            commands.entity(entity).insert(StartPosition(pos));
        }
    }

    if action.pressed(&ToolControls::Confirm) {
        if let Some(world_position) = get_pos(window_query.single(), camera, transform) {
            let pos = Position::from_world(world_position.extend(map.current_level as f32));
            commands.entity(entity).insert(EndPosition(pos));
        }
    }

    if action.just_released(&ToolControls::Confirm) {
        commands.entity(entity).remove::<StartPosition>().remove::<EndPosition>();
    }
}

fn draw_gizmos(mut gizmos: Gizmos, query: Query<(&StartPosition, &EndPosition)>) {
    let Ok((start, end)) = query.get_single() else {
        return;
    };

    let (s, e) = world_bounding_box(start.0, end.0);
    let mut center = (s + e) / 2.0;
    center.z = s.z.max(e.z);
    let size = (s - e).truncate();
    gizmos.rect(center, Quat::IDENTITY, size, GRAY_50);
}

fn world_bounding_box(start: Position, end: Position) -> (Vec3, Vec3) {
    let (min, max) = bounding_box(start, end);
    (min.into_world() - Position::OFFSET, max.into_world() + Position::OFFSET)
}

fn bounding_box(start: Position, end: Position) -> (Position, Position) {
    let max_x = start.x.max(end.x);
    let max_z = start.z.max(end.z);
    let max_elevation = start.elevation.max(end.elevation);

    let min_x = start.x.min(end.x);
    let min_z = start.z.min(end.z);
    let min_elevation = start.elevation.min(end.elevation);

    (Position::new(min_x, min_z, min_elevation), Position::new(max_x, max_z, max_elevation))
}

fn get_pos(window: &Window, camera: &Camera, transform: &GlobalTransform) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(transform, cursor))
        .map(|ray| ray.origin.truncate())
}
