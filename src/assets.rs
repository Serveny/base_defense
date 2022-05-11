use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct StandardAssets {
    #[asset(path = "fonts/Quicksand-Regular.ttf")]
    pub font: Handle<Font>,
}
