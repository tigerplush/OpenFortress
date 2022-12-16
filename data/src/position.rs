use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

use rand::{{thread_rng, Rng}};

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Component, Copy, Debug)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub elevation: i16,
}

const DIRECTIONS: [Position; 6] = [
    // up
    Position { x: 0, y: 0, elevation: 1},
    // down
    Position { x: 0, y: 0, elevation: -1},
    // left
    Position { x: -1, y: 0, elevation: 0},
    // right
    Position { x: 1, y: 0, elevation: 0},
    // forward
    Position { x: 0, y: 1, elevation: 0},
    // backward
    Position { x: 0, y: -1, elevation: 0}
];

impl Position {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Position {
            x: rng.gen_range(-10..10),
            y: rng.gen_range(-10..10),
            elevation: rng.gen_range(-10..10)
        }
    }
}