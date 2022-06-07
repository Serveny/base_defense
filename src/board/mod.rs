pub use self::tile::Tile;
pub use action_board::ActionBoard;
use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};

mod action_board;
mod tile;

const TILE_NEIGHBOR_MATRIX: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// Struct only for holding data, which can be de/serialized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub width: u8,
    pub height: u8,
    pub tiles: Vec<Vec<Tile>>,
}

impl Board {
    pub fn empty(width: u8, height: u8) -> Self {
        Self {
            name: String::new(),
            width,
            height,
            tiles: (0..height)
                .map(|_| (0..width).map(|_| Tile::Empty).collect())
                .collect(),
        }
    }

    pub fn get_tiles(&self, filter: Tile) -> HashSet<UVec2> {
        let mut building_tiles: HashSet<UVec2> = HashSet::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == filter {
                    building_tiles.insert(UVec2::new(x as u32, y as u32));
                }
            }
        }
        building_tiles
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(10, 6)
    }
}
