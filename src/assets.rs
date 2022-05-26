use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct StandardAssets {
    #[asset(path = "fonts/Quicksand-Regular.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "textures/editor-road-start-20.png")]
    pub editor_road_start: Handle<Image>,

    #[asset(path = "textures/editor-road-end-64.png")]
    pub editor_road_end: Handle<Image>,
}
