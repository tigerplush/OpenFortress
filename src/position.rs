use std::ops::{Add, Sub};

use bevy::prelude::*;

#[derive(Clone, Component, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub z: i32,
    pub elevation: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x: {}, z: {}, elevation: {})",
            self.x, self.z, self.elevation
        )
    }
}

impl Position {
    pub const fn new(x: i32, z: i32, elevation: i32) -> Self {
        Self { x, z, elevation }
    }

    pub const fn directions() -> [Self; 6] {
        [
            Self::UP,
            Self::DOWN,
            Self::LEFT,
            Self::RIGHT,
            Self::FORWARD,
            Self::BACKWARD,
        ]
    }

    pub fn neighbors(self) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::new();
        for direction in Self::directions() {
            result.push(self + direction);
        }
        result
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    pub fn world_distance(self, rhs: Self) -> f32 {
        (self - rhs).length() * Self::CONVERSION
    }

    fn dot(self, rhs: Self) -> f32 {
        ((self.x * rhs.x) + (self.elevation * rhs.elevation) + (self.z * rhs.z)) as f32
    }

    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub const CONVERSION: f32 = 32.0;

    pub const OFFSET: Vec3 = Vec3::new(Self::CONVERSION / 2.0, Self::CONVERSION / 2.0, 0.0);

    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const UP: Self = Self::new(0, 0, 1);
    pub const DOWN: Self = Self::new(0, 0, -1);
    pub const LEFT: Self = Self::new(0, -1, 0);
    pub const RIGHT: Self = Self::new(0, 1, 0);
    pub const FORWARD: Self = Self::new(-1, 0, 0);
    pub const BACKWARD: Self = Self::new(1, 0, 0);

    pub fn into_world(self) -> Vec3 {
        Vec3::new(
            self.x as f32 * Self::CONVERSION,
            self.z as f32 * Self::CONVERSION,
            self.elevation as f32,
        )
    }

    pub fn from_world(value: Vec3) -> Self {
        Self {
            x: (value.x / Self::CONVERSION).round() as i32,
            z: (value.y / Self::CONVERSION).round() as i32,
            elevation: value.z as i32,
        }
    }
}

impl Add<Position> for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            elevation: self.elevation.add(rhs.elevation),
            z: self.z.add(rhs.z),
        }
    }
}

impl Sub<Position> for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            elevation: self.elevation.sub(rhs.elevation),
            z: self.z.sub(rhs.z),
        }
    }
}

impl From<(u32, u32, i32)> for Position {
    fn from(value: (u32, u32, i32)) -> Self {
        Position::new(value.0 as i32, value.1 as i32, value.2)
    }
}

#[test]
fn distance() {
    assert_eq!(Position::ZERO.distance(Position::UP), 1.0);
    assert_eq!(Position::ZERO.distance(Position::DOWN), 1.0);
    assert_eq!(Position::LEFT.distance(Position::RIGHT), 2.0);
}
