#![allow(unused)]
use crate::utils::{Building, Tower};
use bevy::prelude::*;

// Place on the board
#[derive(Debug, Clone)]
pub enum Tile {
    // A place where towers can be built
    TowerGround(Option<Tower>),

    // A place where base buildings can be built
    BuildingGround(Option<Building>),

    // The road, enemy must was
    Road,

    // Blocked, unusable place
    Empty,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub tiles: Vec<Vec<Tile>>,
}

impl Board {
    pub fn empty(height: u32, width: u32) -> Self {
        Self {
            width,
            height,
            tiles: (0..width)
                .map(|_| (0..height).map(|_| Tile::Empty).collect())
                .collect(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(12, 20)
    }
}
