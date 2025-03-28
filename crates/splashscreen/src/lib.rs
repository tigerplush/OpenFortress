use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use common::{
    components::image_node_fade::{self, ImageNodeFade},
    states::AppState,
    traits::UiRoot,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Splashscreen), setup)
        .add_systems(
            Update,
            (
                image_node_fade::tick,
                image_node_fade::apply.after(image_node_fade::tick),
                advance_state.after(image_node_fade::tick),
            )
                .run_if(in_state(AppState::Splashscreen)),
        )
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
                    "ui/open_fortress_splashscreen.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest()
                    },
                ),
                ..default()
            },
            ImageNodeFade::default()
        )],
    ));

    commands.spawn((Camera2d, StateScoped(AppState::Splashscreen)));
}

fn advance_state(
    mut next_state: ResMut<NextState<AppState>>,
    animation_query: Query<&ImageNodeFade>,
) {
    if animation_query.iter().all(|anim| anim.elapsed()) {
        next_state.set(AppState::Loading);
    }
}

fn teardown() {}
