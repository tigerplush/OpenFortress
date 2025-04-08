use assets::{font_asset::FontAsset, sound_assets::SoundAsset, ui_panel_asset::UiPanelAsset};
use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use common::traits::AddNamedObserver;

#[derive(Component)]
pub enum UiButton {
    Menu(String),
}

impl UiButton {
    pub fn menu(label: impl Into<String>) -> Self {
        UiButton::Menu(label.into())
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            trigger_interaction_sound_effect.run_if(resource_exists::<SoundAsset>),
            trigger_interaction_color_change,
        ),
    )
    .add_named_observer(add_button, "add_button");
}

fn add_button(
    trigger: Trigger<OnAdd, UiButton>,
    font_asset: Res<FontAsset>,
    panel_asset: Res<UiPanelAsset>,
    query: Query<&UiButton>,
    mut commands: Commands,
) {
    let button = query.get(trigger.target()).unwrap();
    match button {
        UiButton::Menu(label) => {
            commands.entity(trigger.target()).insert((
                ImageNode {
                    image: panel_asset.image.clone_weak(),
                    image_mode: NodeImageMode::Sliced(panel_asset.slicer.clone()),
                    ..default()
                },
                Node {
                    padding: UiRect::axes(Val::Px(48.0), Val::Px(8.0)),
                    ..default()
                },
                Button,
                children![(
                    Text::new(label),
                    TextFont {
                        font: font_asset.font.clone_weak(),
                        font_size: 50.0,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                )],
            ));
        }
    }
}

fn trigger_interaction_sound_effect(
    sound_assets: Res<SoundAsset>,
    query: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &query {
        let source = match interaction {
            Interaction::Hovered => sound_assets.hover.clone_weak(),
            Interaction::Pressed => sound_assets.press.clone_weak(),
            _ => continue,
        };
        commands.spawn((
            Name::new("Button Sound"),
            AudioPlayer::new(source),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn trigger_interaction_color_change(
    mut query: Query<(&mut ImageNode, &Interaction), Changed<Interaction>>,
) {
    for (mut node, interaction) in &mut query {
        let color = match interaction {
            Interaction::None => WHITE.into(),
            Interaction::Hovered => Color::srgba(0.9607843, 0.9607843, 0.9607843, 1.0),
            Interaction::Pressed => Color::srgba(0.7843137, 0.7843137, 0.7843137, 1.0),
        };
        node.color = color;
    }
}
