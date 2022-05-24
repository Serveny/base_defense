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
            tiles: (0..height)
                .map(|_| (0..width).map(|_| Tile::Empty).collect())
                .collect(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(10, 6)
    }
}

const TILE_NEIGHBOR_MATRIX: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

// Validation
impl Board {
    pub fn validate(&self) -> Result<(), &str> {
        let tower_tiles = self.get_tiles(Tile::TowerGround(None));
        let building_tiles = self.get_tiles(Tile::BuildingGround(None));
        let road_tiles = self.get_tiles(Tile::Road);

        if tower_tiles.len() == 0 {
            return Err("Board needs tower tiles");
        }
        if building_tiles.len() == 0 {
            return Err("Board needs building tiles");
        }
        if road_tiles.len() == 0 {
            return Err("Board needs road tiles");
        }

        let edges = BoardEdges::new(&self.tiles);
        if !edges.has_start_point() {
            return Err("Need road starting point at the board edge.");
        }
        if !Self::are_tiles_connected(&road_tiles) {
            return Err("All road tiles must be connected to each other.");
        }
        if !Self::are_tiles_connected(&building_tiles) {
            return Err("All building tiles must be connected to each other.");
        }
        if Self::have_tiles_more_than_max_neighbors(2, &road_tiles) {
            return Err("Only one clear road allowed.");
        }
        if !Self::road_has_valid_end(&road_tiles, &building_tiles) {
            return Err("Road must end surrounded by three building tiles.");
        }

        Ok(())
    }

    fn road_has_valid_end(road_tiles: &HashSet<UVec2>, building_tiles: &HashSet<UVec2>) -> bool {
        for road_tile in road_tiles {
            if Self::get_neighbors(road_tile, building_tiles).len() == 3 {
                return true;
            }
        }
        false
    }

    fn have_tiles_more_than_max_neighbors(max: usize, tiles: &HashSet<UVec2>) -> bool {
        for tile in tiles {
            if Self::get_neighbors(tile, tiles).len() > max {
                return true;
            }
        }
        false
    }

    fn are_tiles_connected(tiles: &HashSet<UVec2>) -> bool {
        let mut connected_tiles: HashSet<UVec2> = HashSet::new();
        if let Some(start) = tiles.iter().last() {
            println!("{:?}", tiles);
            Self::check_neighbors(start.clone(), tiles, &mut connected_tiles);
        }
        println!("cmp {}, {}", tiles.len(), connected_tiles.len());
        tiles.len() == connected_tiles.len()
    }

    fn check_neighbors(pos: UVec2, tiles: &HashSet<UVec2>, linked: &mut HashSet<UVec2>) {
        println!("Add {:?}", pos);
        linked.insert(pos);
        for neighbor in Self::get_neighbors(&pos, &tiles) {
            println!("Check {:?}", neighbor);
            if tiles.get(&neighbor).is_some() && linked.get(&neighbor).is_none() {
                println!("Found {:?}", neighbor);
                Self::check_neighbors(neighbor, tiles, linked);
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

    fn get_neighbors(pos: &UVec2, tiles: &HashSet<UVec2>) -> Vec<UVec2> {
        let mut neighbors = Vec::new();
        for additor in TILE_NEIGHBOR_MATRIX {
            let x = pos.x as i32 + additor.0;
            let y = pos.y as i32 + additor.1;
            if x >= 0 && y >= 0 {
                let neighbor = UVec2::new(x as u32, y as u32);
                if tiles.get(&neighbor).is_some() {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }
}
