use crate::utils::{Building, Tower};
use bevy::{prelude::*, utils::HashSet};
use serde::{Deserialize, Serialize};

const TILE_NEIGHBOR_MATRIX: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

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

    // is tile at one edge of the board and has only one road neighbor, it is the starting point
    pub fn get_road_start_pos<'a>(&self, road_tiles: &'a HashSet<UVec2>) -> Option<&'a UVec2> {
        road_tiles.iter().find(|pos| {
            let (x, y) = (pos.x, pos.y);
            (x == 0 || y == 0 || x == self.width as u32 - 1 || y == self.height as u32 - 1)
                && Self::get_neighbors(pos, road_tiles).len() == 1
        })
    }

    // if tile is surrounded by three building tiles, it is the ending point
    pub fn get_road_end_pos<'a>(
        road_tiles: &'a HashSet<UVec2>,
        building_tiles: &HashSet<UVec2>,
    ) -> Option<&'a UVec2> {
        road_tiles
            .iter()
            .find(|pos| Self::get_neighbors(pos, building_tiles).len() == 3)
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
}

impl Default for Board {
    fn default() -> Self {
        Self::empty(10, 6)
    }
}

// Validation
impl Board {
    pub fn validate(&self) -> Result<(), &str> {
        let tower_tiles = self.get_tiles(Tile::TowerGround(None));
        let building_tiles = self.get_tiles(Tile::BuildingGround(None));
        let road_tiles = self.get_tiles(Tile::Road);

        if tower_tiles.len() < 1 {
            return Err("Need tower tiles");
        }
        if building_tiles.len() < 5 {
            return Err("Need minimal five building tiles");
        }
        if road_tiles.len() < 2 {
            return Err("Need minimal two road tiles");
        }

        if self.get_road_start_pos(&road_tiles).is_none() {
            return Err("Need road starting point at the board edge");
        }
        if !Self::are_tiles_connected(&road_tiles) {
            return Err("All road tiles must be connected to each other");
        }
        if !Self::are_tiles_connected(&building_tiles) {
            return Err("All building tiles must be connected to each other");
        }
        if Self::have_tiles_more_than_max_neighbors(2, &road_tiles) {
            return Err("Only one clear road allowed");
        }
        if Self::get_road_end_pos(&road_tiles, &building_tiles).is_none() {
            return Err("Road must end surrounded by three building tiles");
        }

        Ok(())
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
}
