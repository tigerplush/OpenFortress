use bevy::state::state::States;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    Splashscreen,
    Loading,
    MainMenu,
    WorldGeneration,
    MainGame,
}
