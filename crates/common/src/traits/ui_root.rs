use bevy::prelude::*;

pub trait UiRoot {
    fn ui_root(&mut self) -> EntityCommands;
}

impl UiRoot for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("Ui Root"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
    }
}