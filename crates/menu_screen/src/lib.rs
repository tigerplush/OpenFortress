use assets::background_asset::MenuBackgroundAsset;
use bevy::prelude::*;
use common::{states::AppState, traits::UiRoot};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup);
}

fn setup(background: Res<MenuBackgroundAsset>, mut commands: Commands) {
    commands.ui_root().insert((
        StateScoped(AppState::MainMenu),
        ImageNode {
            image: background.sprite.clone_weak(),
            ..default()
        },
    ));

    commands.spawn((Camera2d, StateScoped(AppState::MainMenu)));
}
