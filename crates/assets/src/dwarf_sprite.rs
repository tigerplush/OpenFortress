use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct DwarfSpriteAsset {
    pub sprite: Handle<Image>,
    pub texture_atlas: TextureAtlas,
}

impl DwarfSpriteAsset {
    const PATH: &'static str = "Dwarf Sprite Sheet 1.3v.png";
}

impl FromWorld for DwarfSpriteAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 8, 8, None, None);
            let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
            layouts.add(layout)
        };
        let assets = world.resource::<AssetServer>();
        DwarfSpriteAsset {
            sprite: assets.load_with_settings(
                DwarfSpriteAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            texture_atlas: TextureAtlas {
                index: 0,
                layout: layout_handle
            }
        }
    }
}
