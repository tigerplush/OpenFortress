use animation::{AnimationConfig, AnimationState, Frames};
use assets::dwarf_sprite::DwarfSpriteAsset;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_dwarf);
}

#[derive(Component)]
#[require(AnimationConfig)]
pub struct Dwarf;

fn on_add_dwarf(
    trigger: Trigger<OnAdd, Dwarf>,
    dwarf: Res<DwarfSpriteAsset>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).insert((
        Sprite {
            image: dwarf.sprite.clone_weak(),
            texture_atlas: Some(dwarf.texture_atlas.clone()),
            ..default()
        },
        AnimationState::new(DwarfAnimationState::default()),
    ));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
enum DwarfAnimationState {
    #[default]
    Idling,
    Walking,
}

impl Frames for DwarfAnimationState {
    fn frames(&self) -> (usize, usize) {
        match self {
            DwarfAnimationState::Idling => (0, 4),
            DwarfAnimationState::Walking => (8, 15),
        }
    }
}
