use bevy::prelude::*;

pub mod components;
pub mod constants;
pub mod functions;
pub mod resources;
pub mod states;
pub mod systems;
pub mod traits;
pub mod types;
use components::image_node_fade::ImageNodeFade;
use states::AppState;

pub fn plugin(app: &mut App) {
    app.init_state::<AppState>()
        .add_systems(PostUpdate, systems::apply_world_coordinates)
        .register_type::<ImageNodeFade>();
}
