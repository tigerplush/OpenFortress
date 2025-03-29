use std::time::Duration;

use assets::dwarf_sprite::DwarfSpriteAsset;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_dwarf).add_systems(Update, (tick, animate.after(tick)));
}

#[derive(Component)]
#[require(Sprite, AnimationConfig, AnimationState)]
pub struct Dwarf;

fn on_add_dwarf(trigger: Trigger<OnAdd, Dwarf>, dwarf: Res<DwarfSpriteAsset>, mut query: Query<&mut Sprite, With<Dwarf>>) {
    if let Ok(mut sprite) = query.get_mut(trigger.target()) {
        sprite.image = dwarf.sprite.clone_weak();
        sprite.texture_atlas = Some(dwarf.texture_atlas.clone())
    }
}

fn tick(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &AnimationState)>) {
    for (mut config, state) in &mut query {
        config.timer.tick(time.delta());
        if config.timer.just_finished() {
            config.current_frame += 1;
        }
        let (start, end) = state.frames();
        if config.current_frame < start || config.current_frame > end {
            config.current_frame = start;
        }
    }
}

fn animate(mut query: Query<(&mut Sprite, &AnimationConfig), Changed<AnimationConfig>>) {
    for (mut sprite, config) in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = config.current_frame;
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct AnimationConfig {
    current_frame: usize,
    fps: u8,
    timer: Timer,
}

impl AnimationConfig {
    fn new(fps: u8) -> Self {
        AnimationConfig {
            current_frame: 0,
            fps,
            timer: Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Repeating),
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        AnimationConfig::new(12)
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
enum AnimationState {
    #[default]
    Idling,
    Walking,
}

impl AnimationState {
    fn frames(&self) -> (usize, usize) {
        match self {
            AnimationState::Idling => (0, 4),
            AnimationState::Walking => (8, 15),
        }
    }
}
