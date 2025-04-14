use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct TilesetAsset {
    pub soil_tileset: Handle<Image>,
    pub fog_tileset: Handle<Image>,
    pub water_tileset: Handle<Image>,
}

impl TilesetAsset {
    const SOIL_PATH: &'static str = "tilesets/tileset_soil.png";
    const FOG_PATH: &'static str = "tilesets/tileset_fog.png";
    const WATER_PATH: &'static str = "tilesets/tileset_water.png";
}

impl FromWorld for TilesetAsset {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        TilesetAsset {
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
            water_tileset: assets.load_with_settings(
                TilesetAsset::WATER_PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
        }
    }
}
