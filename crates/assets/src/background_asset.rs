use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct MenuBackgroundAsset {
    pub sprite: Handle<Image>,
}

impl MenuBackgroundAsset {
    const PATH: &'static str = "open_fortress_main_bg.png";
}

impl FromWorld for MenuBackgroundAsset {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        MenuBackgroundAsset {
            sprite: assets.load_with_settings(
                MenuBackgroundAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
