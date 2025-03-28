use bevy::prelude::*;

pub mod states;
pub mod traits;
use states::AppState;

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>();
}
