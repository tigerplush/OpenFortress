use assets::resource_handles::ResourceHandles;
use bevy::prelude::*;
use common::{states::AppState, traits::UiRoot};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Loading), setup)
        .add_systems(
            Update,
            continue_to_title_screen.run_if(all_assets_loaded.and(in_state(AppState::Loading))),
        )
        .add_systems(OnExit(AppState::Loading), teardown);
}

fn setup(mut commands: Commands) {
    commands.ui_root().insert((
        Name::new("Loading Screen"),
        StateScoped(AppState::Loading),
        children![(
            Node {
                justify_content: JustifyContent::Center,
                ..default()
            },
            Text::new("Loading..."),
            TextFont::from_font_size(24.0),
            TextColor(Color::srgb(0.867, 0.827, 0.412)),
        )]
    ));

    commands.spawn((Camera2d, StateScoped(AppState::Loading)));
}

fn continue_to_title_screen(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}

fn teardown() {
}
