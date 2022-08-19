use super::TilesPerSecond;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Speed {
    pub normal: TilesPerSecond,
    pub current: TilesPerSecond,
    pub target: TilesPerSecond,
}

impl Speed {
    pub fn new(normal: TilesPerSecond) -> Self {
        Self {
            normal,
            current: normal,
            target: normal,
        }
    }
}
