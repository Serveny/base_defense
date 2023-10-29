use serde::{Deserialize, Serialize};

// Place on the board
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Tile {
    // A place where towers can be built
    #[default]
    TowerGround,

    // A place where base buildings can be built
    BuildingGround,

    // The road, enemy must was
    Road,

    // Blocked, unusable place
    Empty,
}

impl Tile {
    pub fn is_buildable(&self) -> bool {
        matches!(self, Tile::TowerGround | Tile::BuildingGround)
    }
}
