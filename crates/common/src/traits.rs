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

pub trait Neighbors<T> {
    /// Returns all neighbors with their squared cost.
    /// 
    /// The order is:
    /// ```
    /// NW, N, NE,
    ///  W,     E,
    /// SW, S, SE
    /// ```
    fn neighbors(&self) -> Vec<(T, u32)>;
}

#[rustfmt::skip]
impl Neighbors<IVec3> for IVec3 {
    fn neighbors(&self) -> Vec<(IVec3, u32)> {
        vec![
            (self + IVec3::new(-1,  1,  0), 2), (self + IVec3::new( 0,  1,  0), 1), (self + IVec3::new( 1,  1,  0), 2),
            (self + IVec3::new(-1,  0,  0), 1),                                     (self + IVec3::new( 1,  0,  0), 1),
            (self + IVec3::new(-1, -1,  0), 2), (self + IVec3::new( 0, -1,  0), 1), (self + IVec3::new( 1, -1,  0), 2),
        ]
    }
}
