use bevy::log;
use bevy::prelude::*;
use bevy::diagnostic::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

#[derive(Component)]
pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(Self::add_fps);
        app.add_system(Self::update);
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<FpsComponent>();
        }
        log::info!("Loaded FpsPlugin");
    }
}

impl FpsPlugin {
    fn add_fps(mut commands: Commands) {
        commands.spawn(FpsComponent {fps: 0.0})
            .insert(Name::new("FPS"));
    }

    fn update(
        diagnostics: Res<Diagnostics>,
        mut query: Query<&mut FpsComponent>
    ) {
        for mut fps_component in query.iter_mut() {
            if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(raw) = fps.value() {
                    fps_component.fps = raw;
                }
            }
        }
    }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug)]
struct FpsComponent {
    pub fps: f64,
}