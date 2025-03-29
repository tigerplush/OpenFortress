use bevy::prelude::*;
use common::states::AppState;
use dwarf::Dwarf;
use map_generation::WorldMap;

pub fn plugin(app: &mut App) {
    app.add_plugins((dwarf::plugin, map_generation::plugin))
        .add_systems(OnEnter(AppState::MainGame), setup);
}

fn setup(mut commands: Commands) {
    // for _ in 0..7 {
    commands.spawn(Dwarf);
    // }

    commands.spawn(WorldMap);
    commands.spawn(Camera2d);
}
