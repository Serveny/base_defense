use std::cmp::Ordering;

pub use self::cache::BoardCache;
pub use self::tile::Tile;
use bevy::prelude::*;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

mod cache;
pub mod step;
mod tile;
pub mod visualisation;

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

    pub fn get_tile_mut(&mut self, pos: &UVec2) -> Option<&mut Tile> {
        if self.get_tile(pos).is_some() {
            Some(&mut self.tiles[pos.y as usize][pos.x as usize])
        } else {
            None
        }
    }

    pub fn get_tile(&self, pos: &UVec2) -> Option<&Tile> {
        if let Some(row) = self.tiles.get(pos.y as usize) {
            return row.get(pos.x as usize);
        }
        None
    }

    pub fn get_tiles(&self, filter: Tile) -> IndexSet<UVec2> {
        let mut tiles: IndexSet<UVec2> = IndexSet::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == filter {
                    tiles.insert(UVec2::new(x as u32, y as u32));
                }
            }
        }
        tiles
    }

    pub fn change_size(&mut self, new_width: u8, new_heigth: u8) {
        // Add/reduce height
        match new_heigth.cmp(&self.height) {
            Ordering::Less => {
                let to_del = self.height - new_heigth;
                for _ in 0..to_del {
                    self.tiles.pop();
                }
            }
            Ordering::Equal => (),
            Ordering::Greater => {
                let to_add = new_heigth - self.height;
                for _ in 0..to_add {
                    let mut row = Vec::new();
                    for _ in 0..self.width {
                        row.push(Tile::Empty);
                    }
                    self.tiles.push(row);
                }
            }
        }

        // Add/reduce width
        match new_width.cmp(&self.width) {
            Ordering::Less => {
                let to_del = self.width - new_width;
                for row in &mut self.tiles {
                    for _ in 0..to_del {
                        row.pop();
                    }
                }
            }
            Ordering::Equal => (),
            Ordering::Greater => {
                let to_add = new_width - self.width;
                for row in &mut self.tiles {
                    for _ in 0..to_add {
                        row.push(Tile::Empty);
                    }
                }
            }
        }

        self.width = new_width;
        self.height = new_heigth;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(10, 6)
    }
}
