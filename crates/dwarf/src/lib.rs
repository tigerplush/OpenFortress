use animation::{AnimationConfig, AnimationState, Frames};
use assets::dwarf_sprite::DwarfSpriteAsset;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_dwarf);
}

#[derive(Component)]
#[require(Sprite, AnimationConfig)]
pub struct Dwarf;

fn on_add_dwarf(
    trigger: Trigger<OnAdd, Dwarf>,
    dwarf: Res<DwarfSpriteAsset>,
    mut query: Query<&mut Sprite, With<Dwarf>>,
    mut commands: Commands,
) {
    if let Ok(mut sprite) = query.get_mut(trigger.target()) {
        sprite.image = dwarf.sprite.clone_weak();
        sprite.texture_atlas = Some(dwarf.texture_atlas.clone())
    }
    commands.entity(trigger.target()).insert(AnimationState::new(DwarfAnimationState::default()));
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
