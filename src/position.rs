use std::ops::{Sub, Add};

use bevy::prelude::*;

#[derive(Clone, Component, Copy, Eq, Hash, PartialEq)]
pub struct Position {
    x: i32,
    z: i32,
    elevation: i32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, z: {}, elevation: {})", self.x, self.z, self.elevation)
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

    fn dot(self, rhs: Self) -> f32 {
        ((self.x * rhs.x) + (self.elevation * rhs.elevation) + (self.z * rhs.z)) as f32
    }

    fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub const ZERO: Self = Self::new(0, 0, 0);
    pub const UP: Self = Self::new(0, 0, 1);
    pub const DOWN: Self = Self::new(0, 0, -1);
    pub const LEFT: Self = Self::new(0, -1, 0);
    pub const RIGHT: Self = Self::new(0, 1, 0);
    pub const FORWARD: Self = Self::new(-1, 0, 0);
    pub const BACKWARD: Self = Self::new(1, 0, 0);
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

#[test]
fn distance() {
    assert_eq!(Position::ZERO.distance(Position::UP), 1.0);
    assert_eq!(Position::ZERO.distance(Position::DOWN), 1.0);
    assert_eq!(Position::LEFT.distance(Position::RIGHT), 2.0);
}
