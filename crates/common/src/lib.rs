use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod functions;
pub mod resources;
pub mod states;
pub mod traits;
pub mod types;
use components::image_node_fade::ImageNodeFade;
use states::AppState;

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .register_type::<ImageNodeFade>();
}
