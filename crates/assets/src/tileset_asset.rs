use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub soil_tileset: Handle<Image>,
    pub fog_tileset: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}

impl TilesetAsset {
    const PATH: &'static str = "tilesets/tileset.png";
    const SOIL_PATH: &'static str = "tilesets/tileset_soil.png";
    const FOG_PATH: &'static str = "tilesets/tileset_fog.png";
}

impl FromWorld for TilesetAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 48, 1, None, None);
            let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
            layouts.add(layout)
        };
        let assets = world.resource::<AssetServer>();
        TilesetAsset {
            image: assets.load_with_settings(
                TilesetAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            soil_tileset: assets.load_with_settings(
                TilesetAsset::SOIL_PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            fog_tileset: assets.load_with_settings(
                TilesetAsset::FOG_PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            layout_handle,
        }
    }
}
