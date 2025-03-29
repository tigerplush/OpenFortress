use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct TilesetAsset {
    pub image: Handle<Image>,
    pub layout_handle: Handle<TextureAtlasLayout>,
}

pub const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);

pub struct TileType;

impl TileType {
    pub const GRASS: usize = 1 * 14 + 1;
    pub const WATER: usize = 1 * 14 + 6;
    pub const LAVA: usize = 1 * 14 + 11;
    pub const BRIGHT_GRASS: usize = 10 * 14 + 1;
    pub const DIRT: usize = 10 * 14 + 6;
    pub const FIELD: usize = 10 * 14 + 11;
    pub const NONE: usize = 23 * 14 + 14;
}

impl TilesetAsset {
    const PATH: &'static str = "1_terrain.png";
}

impl FromWorld for TilesetAsset {
    fn from_world(world: &mut World) -> Self {
        let layout_handle = {
            let layout = TextureAtlasLayout::from_grid(UVec2::new(32, 32), 14, 24, None, None);
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
            layout_handle,
        }
    }
}
