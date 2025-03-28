use bevy::prelude::*;

#[derive(Asset, Clone, Resource, TypePath)]
pub struct SoundAsset {
    pub hover: Handle<AudioSource>,
    pub press: Handle<AudioSource>,
}

impl SoundAsset {
    const HOVER: &'static str = "sounds/Retro2.ogg";
    const PRESS: &'static str = "sounds/Retro6.ogg";
}

impl FromWorld for SoundAsset {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        SoundAsset {
            hover: assets.load(SoundAsset::HOVER),
            press: assets.load(SoundAsset::PRESS),
        }
    }
}
