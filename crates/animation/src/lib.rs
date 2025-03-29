use std::{sync::Arc, time::Duration};

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (tick, animate.after(tick)));
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
pub struct AnimationConfig {
    current_frame: usize,
    fps: u8,
    timer: Timer,
}

impl AnimationConfig {
    fn new(fps: u8) -> Self {
        AnimationConfig {
            current_frame: 0,
            fps,
            timer: Timer::new(
                Duration::from_secs_f32(1.0 / (fps as f32)),
                TimerMode::Repeating,
            ),
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        AnimationConfig::new(12)
    }
}

pub trait Frames: Send + Sync {
    fn frames(&self) -> (usize, usize);
}

#[derive(Component, Deref)]
pub struct AnimationState(Arc<dyn Frames>);

impl AnimationState {
    pub fn new(component: impl Frames + 'static) -> Self {
        AnimationState(Arc::new(component))
    }
}
