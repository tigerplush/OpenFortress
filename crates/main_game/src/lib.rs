use assets::icon_asset::IconAsset;
use bevy::prelude::*;
use camera::CameraPlugin;
use common::{constants::TILE_SIZE, states::AppState, traits::AddNamedObserver};
use dwarf::Dwarf;
use work::WorkOrder;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        CameraPlugin::default(),
        designations::plugin,
        dwarf::plugin,
        map_generation::plugin,
        pathfinding::plugin,
        work::plugin,
    ))
    .add_systems(OnEnter(AppState::MainGame), setup)
    .add_named_observer(add_vis_to_work_order, "add_vis_to_work_order");
}

fn setup(mut commands: Commands) {
    for i in 0..1 {
        commands.spawn((
            Dwarf,
            Transform::from_xyz((i - 4) as f32 * TILE_SIZE.x, 0.0, 0.0),
        ));
    }
}

fn add_vis_to_work_order(
    trigger: Trigger<OnAdd, WorkOrder>,
    icon_asset: Res<IconAsset>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.target())
        .insert(icon_asset.sprite(IconAsset::SHOVEL));
}
