use bevy::prelude::*;

pub trait AsVec2 {
    fn as_vec2(&self) -> Vec2;
}

impl AsVec2 for (u32, u32) {
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}