use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct IconAsset {
    pub image: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}

impl IconAsset {
    const PATH: &'static str = "32x32.png";
    pub const SHOVEL: usize = 57 * 16;

    pub fn sprite(&self, index: usize) -> Sprite {
        Sprite {
            image: self.image.clone_weak(),
            color: Color::default().with_alpha(0.8),
            texture_atlas: Some(TextureAtlas {
                layout: self.layout_handle.clone_weak(),
                index,
            }),
            ..default()
        }
    }
}

impl FromWorld for IconAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 16, 137, None, None);
            let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
            layouts.add(layout)
        };
        let assets = world.resource::<AssetServer>();
        IconAsset {
            image: assets.load_with_settings(
                IconAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            layout_handle,
        }
    }
}
