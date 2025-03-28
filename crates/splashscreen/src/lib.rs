use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use common::{states::AppState, traits::UiRoot};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splashscreen), setup)
        .add_systems(OnExit(AppState::Splashscreen), teardown);
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.ui_root().insert((
        Name::new("Splash Screen"),
        StateScoped(AppState::Splashscreen),
        children![(
            Name::new("Splash Image"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode {
                image: asset_server.load_with_settings(
                    "open_fortress_splashscreen.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest()
                    },
                ),
                ..default()
            }
        )],
    ));

    commands.spawn((Camera2d, StateScoped(AppState::Splashscreen)));
}

fn teardown() {}
