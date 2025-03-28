use assets::background_asset::MenuBackgroundAsset;
use bevy::prelude::*;
use common::{states::AppState, traits::UiRoot};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup);
}

fn setup(background: Res<MenuBackgroundAsset>, mut commands: Commands) {
    commands
        .ui_root()
        .insert((
            StateScoped(AppState::MainMenu),
            ImageNode {
                image: background.sprite.clone_weak(),
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn(ui::UiButton::menu("START"))
                .observe(on_press_start);
        });

    commands.spawn((Camera2d, StateScoped(AppState::MainMenu)));
}

fn on_press_start(_trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainGame);
}
