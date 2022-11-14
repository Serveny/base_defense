use self::settings::{DisplayQuality, Volume};
use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    pub display_quality: DisplayQuality,
    pub volume: Volume,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            display_quality: DisplayQuality::High,
            volume: Volume(100),
        }
    }
}

pub mod settings {
    use bevy::prelude::*;

    // One of the two settings that can be set through the menu. It will be a resource in the app
    #[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
    pub enum DisplayQuality {
        // Low,
        // Medium,
        High,
    }

    // One of the two settings that can be set through the menu. It will be a resource in the app
    #[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
    pub struct Volume(pub u8);
}
