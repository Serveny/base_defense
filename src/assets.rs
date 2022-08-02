use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct StandardAssets {
    #[asset(path = "fonts/Quicksand-Regular.ttf")]
    pub font: Handle<Font>,
}
