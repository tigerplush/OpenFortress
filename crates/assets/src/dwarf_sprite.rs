use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct DwarfSpriteAsset {
    pub sprite: Handle<Image>,
}

impl DwarfSpriteAsset {
    const PATH: &'static str = "Dwarf Sprite Sheet 1.3v.png";
}

impl FromWorld for DwarfSpriteAsset {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        DwarfSpriteAsset {
            sprite: assets.load_with_settings(
                DwarfSpriteAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
