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

pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

impl AsVec2 for (u32, u32) {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}
