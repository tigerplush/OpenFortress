use bevy::log;
use bevy::prelude::*;

pub struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, _app: &mut App) {
        log::info!("Loaded DwarfPlugin");
    }
}