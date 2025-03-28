use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ImageNodeFade {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFade {
    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }

    pub fn elapsed(&self) -> bool {
        self.t >= self.total_duration
    }
}

impl Default for ImageNodeFade {
    fn default() -> Self {
        ImageNodeFade {
            total_duration: 1.8,
            fade_duration: 0.6,
            t: 0.0,
        }
    }
}

pub fn tick(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFade>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

pub fn apply(mut animation_query: Query<(&ImageNodeFade, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha());
    }
}
