use bevy::prelude::*;

#[derive(Asset, Clone, Resource, TypePath)]
pub struct FontAsset {
    pub font: Handle<Font>,
}

impl FontAsset {
    const PATH: &'static str = "runic.ttf";
}

impl FromWorld for FontAsset {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        FontAsset {
            font: assets.load(FontAsset::PATH),
        }
    }
}
