use crate::utils::{towers::Tower, Building};
use serde::{Deserialize, Serialize};

// Place on the board
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
