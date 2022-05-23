#![allow(unused)]
use std::{collections::HashMap, error::Error};

use crate::utils::{Building, Tower};
use bevy::{prelude::*, utils::HashSet};
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

struct BoardEdges<'a> {
    pub north: Vec<&'a Tile>,
    pub east: Vec<&'a Tile>,
    pub south: Vec<&'a Tile>,
    pub west: Vec<&'a Tile>,
}

impl<'a> BoardEdges<'a> {
    pub fn new(tiles: &'a Vec<Vec<Tile>>) -> Self {
        Self {
            north: tiles.first().unwrap().iter().map(|tile| tile).collect(),
            east: tiles.iter().map(|row| row.last().unwrap()).collect(),
            south: tiles.last().unwrap().iter().map(|tile| tile).collect(),
            west: tiles.iter().map(|row| row.first().unwrap()).collect(),
        }
    }

    pub fn has_start_point(&self) -> bool {
        self.north.contains(&&Tile::Road)
            || self.east.contains(&&Tile::Road)
            || self.south.contains(&&Tile::Road)
            || self.west.contains(&&Tile::Road)
    }
}

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
            tiles: (0..width)
                .map(|_| (0..height).map(|_| Tile::Empty).collect())
                .collect(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(10, 6)
    }
}

// Validation
impl Board {
    fn is_valid(&self) -> Result<(), &str> {
        let edges = BoardEdges::new(&self.tiles);
        if !edges.has_start_point() {
            return Err("No road starting point at the board edge.");
        }
        if !self.are_all_building_tiles_connected() {
            return Err("Not all building tiles are connected to each other.");
        }

        Ok(())
    }

    fn are_all_building_tiles_connected(&self) -> bool {
        let building_tiles = self.get_tiles(Tile::BuildingGround(None));
        let mut connected_tiles: HashSet<UVec2> = HashSet::new();
        false
    }

    fn check_neighbors(pos: UVec2, tiles: &HashSet<UVec2>, linked: &mut HashSet<UVec2>) {
        let additors = [
            IVec2::new(1, 0),
            IVec2::new(0, 1),
            IVec2::new(-1, 0),
            IVec2::new(0, -1),
        ];
        for additor in additors {
            let x = pos.x as i32 + additor.x;
            let y = pos.y as i32 + additor.y;
            if x > 0 && y > 0 {
                let neighbor = UVec2::new(x as u32, y as u32);
                if tiles.get(&neighbor).is_some() && linked.get(&neighbor).is_none() {
                    linked.insert(neighbor);
                    Board::check_neighbors(neighbor, tiles, linked);
                }
            }
        }
    }

    fn get_tiles(&self, filter: Tile) -> HashSet<UVec2> {
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

    fn is_neighor(one: (u8, u8), two: (u8, u8)) -> bool {
        let distance_x = if one.0 >= two.0 {
            one.0 - two.0
        } else {
            two.0 - one.0
        };
        let distance_y = if one.1 >= two.1 {
            one.1 - two.1
        } else {
            two.1 - one.1
        };
        distance_x + distance_y == 1
    }
}
