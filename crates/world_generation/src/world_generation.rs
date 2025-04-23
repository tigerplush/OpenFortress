use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};
use common::states::AppState;
use map_generation::map_generation::WorldGenerationSettings;

pub fn plugin(app: &mut App) {
    app.insert_resource(WorldGenerationSettings::default())
        .add_systems(
            Update,
            handle_ui.run_if(in_state(AppState::WorldGeneration)),
        );
}

fn handle_ui(
    mut world_generation_settings: ResMut<WorldGenerationSettings>,
    context: Single<&mut EguiContext, With<PrimaryWindow>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut egui_context = context.into_inner().clone();
    egui::Window::new("World Generation Settings").show(egui_context.get_mut(), |ui| {
        ui.add(egui::DragValue::new(&mut world_generation_settings.seed).prefix("Seed: "));
        if ui.button("Generate World").clicked() {
            next_state.set(AppState::MainGame);
        }
    });
}
