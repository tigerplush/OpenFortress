use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};

use crate::{BrushSettings, MouseActions};

pub(crate) fn brushes(
    mut brush_settings: ResMut<BrushSettings>,
    context: Single<&mut EguiContext, With<PrimaryWindow>>,
) {
    let mut egui_context = context.into_inner().clone();
    egui::TopBottomPanel::bottom("brushes")
        .default_height(50.0)
        .show(egui_context.get_mut(), |ui| {
            ui.heading("Brushes");

            if ui
                .selectable_label(brush_settings.current_action == MouseActions::Dig, "Dig")
                .clicked()
            {
                let new_action = match brush_settings.current_action {
                    MouseActions::Dig => MouseActions::None,
                    _ => MouseActions::Dig,
                };
                brush_settings.current_action = new_action;
            }
            ui.allocate_space(ui.available_size());
        });
}
