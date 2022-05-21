#![allow(unused)]
use std::error::Error;

use crate::utils::{Building, Tower};
use bevy::prelude::*;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub name: String,
    pub height: u32,
    pub width: u32,
    pub tiles: Vec<Vec<Tile>>,
}

impl Board {
    pub fn empty(height: u32, width: u32) -> Self {
        Self {
            name: String::new(),
            width,
            height,
            tiles: (0..width)
                .map(|_| (0..height).map(|_| Tile::Empty).collect())
                .collect(),
        }
    }

    pub fn is_valid(&self) -> Result<(), &str> {
        Ok(())
    }

    fn has_way_startpoint(&self) -> Result<(), &str> {
        todo!();
        // let edge_tiles = [
        //     self.tiles.first().ok_or("No north row")?,
        //     self.tiles
        //         .iter()
        //         .map(|row| row.last().ok("No east row"))
        //         .collect::<Vec<Tile>>(),
        //     self.tiles.last().ok_or("No south row")?,
        //     self.tiles
        //         .iter()
        //         .map(|row| row.first().ok_or("No west row"))
        //         .collect()?,
        // ];
        // let north_row = self.tiles.first().ok_or("No north row")?;
        // let east_row: Vec<Tile> = self.tiles.iter().map(|row| row.last()).collect();
        // let south_row = self.tiles.last().ok_or("No south row")?;
        // let west_row: Vec<Tile> = self
        //     .tiles
        //     .iter()
        //     .map(|row| row.first().ok_or("No west row"))
        //     .collect()?;

        // // Has way start point at at least one side of the board
        // if north_row.contains(&Tile::Road)
        //     || east_row.contains(&Tile::Road)
        //     || south_row.contains(&Tile::Road)
        //     || west_row.contains(&Tile::Road)
        // {
        //     Ok(())
        // } else {
        //     Err("No way starting point.")
        // }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(6, 10)
    }
}
