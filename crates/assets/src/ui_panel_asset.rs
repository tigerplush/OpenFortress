use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

#[derive(Asset, Clone, Resource, TypePath)]
pub struct UiPanelAsset {
    pub image: Handle<Image>,
    pub slicer: TextureSlicer,
}

impl UiPanelAsset {
    const PATH: &'static str = "panel-003.png";
}

impl FromWorld for UiPanelAsset {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        UiPanelAsset {
            image: assets.load_with_settings(
                UiPanelAsset::PATH,
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            slicer: TextureSlicer {
                border: BorderRect::all(16.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 1.0,
            },
        }
    }
}
