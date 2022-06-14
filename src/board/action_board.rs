use super::{tile::Tile, Board, TILE_NEIGHBOR_MATRIX};
use bevy::{prelude::*, utils::HashSet};

// Board with meta data and edit actions
#[derive(Default, Clone)]
pub struct ActionBoard {
    board: Board,
    tower_tile_posis: HashSet<UVec2>,
    building_tile_posis: HashSet<UVec2>,
    road_tile_posis: HashSet<UVec2>,
    road_start: Option<UVec2>,
    road_end: Option<UVec2>,
}

impl ActionBoard {
    pub fn new(board: Board) -> Self {
        let tower_tile_posis = board.get_tiles(Tile::TowerGround(None));
        let building_tile_posis = board.get_tiles(Tile::BuildingGround(None));
        let road_tile_posis = board.get_tiles(Tile::Road);
        let road_start = Self::road_start_pos_from(&road_tile_posis, board.width, board.height);
        let road_end = Self::road_end_pos_from(&road_tile_posis, &building_tile_posis);

        Self {
            board,
            tower_tile_posis,
            building_tile_posis,
            road_tile_posis,
            road_start,
            road_end,
        }
    }

    pub fn empty(width: u8, height: u8) -> Self {
        Self {
            board: Board::empty(width, height),
            tower_tile_posis: HashSet::new(),
            building_tile_posis: HashSet::new(),
            road_tile_posis: HashSet::new(),
            road_start: None,
            road_end: None,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn name(&self) -> &str {
        &self.board.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.board.name
    }

    pub fn change_size(&mut self, new_width: u8, new_heigth: u8) {
        let mut board = &mut self.board;

        // Add/reduce width
        if new_width > board.width {
            let to_add = new_width - board.width;
            for row in &mut board.tiles {
                for _ in 0..to_add {
                    row.push(Tile::Empty);
                }
            }
        } else if new_width < board.width {
            let to_del = board.width - new_width;
            for row in &mut board.tiles {
                for _ in 0..to_del {
                    row.pop();
                }
            }
        }

        // Add/reduce height
        if new_heigth > board.height {
            let to_add = new_heigth - board.height;
            for _ in 0..to_add {
                let mut row = Vec::new();
                for _ in 0..board.width {
                    row.push(Tile::Empty);
                }
                board.tiles.push(row);
            }
        } else if new_heigth < board.height {
            let to_del = board.height - new_heigth;
            for _ in 0..to_del {
                board.tiles.pop();
            }
        }

        board.width = new_width;
        board.height = new_heigth;

        self.refill_tiles_posis();
    }

    pub fn get_tile(&self, pos: UVec2) -> &Tile {
        &self.board.tiles[pos.y as usize][pos.x as usize]
    }

    pub fn set_tile(&mut self, pos: UVec2, tile_to: Tile) {
        let tile = self.board.tiles[pos.y as usize][pos.x as usize].clone();
        self.remove_tile_pos(&pos, &tile);
        self.insert_tile_pos(pos, &tile_to);
        self.board.tiles[pos.y as usize][pos.x as usize] = tile_to;
        self.calc_road_data();
    }

    fn calc_road_data(&mut self) {
        self.road_start =
            Self::road_start_pos_from(&self.road_tile_posis, self.board.width, self.board.height);
        self.road_end = Self::road_end_pos_from(&self.road_tile_posis, &self.building_tile_posis);
    }

    fn remove_tile_pos(&mut self, pos: &UVec2, tile: &Tile) {
        if let Some(new_posis) = self.get_tile_posis_mut(tile) {
            new_posis.remove(pos);
        }
    }
    fn insert_tile_pos(&mut self, pos: UVec2, tile: &Tile) {
        if let Some(new_posis) = self.get_tile_posis_mut(tile) {
            new_posis.insert(pos);
        }
    }

    pub fn road_start_pos(&self) -> &Option<UVec2> {
        &self.road_start
        //Self::road_start_pos_from(&self.road_tile_posis, self.board.width, self.board.height)
    }

    // is tile at one edge of the board and has only one road neighbor, it is the starting point
    fn road_start_pos_from(
        road_tile_posis: &HashSet<UVec2>,
        board_width: u8,
        board_heigt: u8,
    ) -> Option<UVec2> {
        for pos in road_tile_posis {
            let (x, y) = (pos.x, pos.y);
            if (x == 0 || y == 0 || x == board_width as u32 - 1 || y == board_heigt as u32 - 1)
                && Self::get_neighbors(pos, road_tile_posis).len() == 1
            {
                return Some(pos.clone());
            }
        }
        None
    }

    pub fn road_end_pos(&self) -> &Option<UVec2> {
        &self.road_end
        //Self::road_end_pos_from(&self.road_tile_posis, &self.building_tile_posis)
    }

    // if tile is surrounded by three building tiles, it is the ending point
    pub fn road_end_pos_from(
        road_tile_posis: &HashSet<UVec2>,
        building_tile_posis: &HashSet<UVec2>,
    ) -> Option<UVec2> {
        for pos in road_tile_posis {
            if Self::get_neighbors(pos, building_tile_posis).len() == 3 {
                return Some(pos.clone());
            }
        }
        None
    }

    fn refill_tiles_posis(&mut self) {
        self.tower_tile_posis = self.board.get_tiles(Tile::TowerGround(None));
        self.building_tile_posis = self.board.get_tiles(Tile::BuildingGround(None));
        self.road_tile_posis = self.board.get_tiles(Tile::Road);
    }
    pub fn tower_tile_posis(&self) -> &HashSet<UVec2> {
        &self.tower_tile_posis
    }
    pub fn building_tile_posis(&self) -> &HashSet<UVec2> {
        &self.building_tile_posis
    }
    pub fn road_tile_posis(&self) -> &HashSet<UVec2> {
        &self.road_tile_posis
    }
    fn get_tile_posis_mut(&mut self, tile: &Tile) -> Option<&mut HashSet<UVec2>> {
        match tile {
            Tile::TowerGround(_) => Some(&mut self.tower_tile_posis),
            Tile::BuildingGround(_) => Some(&mut self.building_tile_posis),
            Tile::Road => Some(&mut self.road_tile_posis),
            Tile::Empty => None,
        }
    }

    pub fn validate(&self) -> Result<(), &str> {
        if self.tower_tile_posis.len() < 1 {
            return Err("Need tower tiles");
        }
        if self.building_tile_posis.len() < 5 {
            return Err("Need minimal five building tiles");
        }
        if self.road_tile_posis.len() < 2 {
            return Err("Need minimal two road tiles");
        }

        if self.road_start_pos().is_none() {
            return Err("Need road starting point at the board edge");
        }
        if !Self::are_tiles_connected(&self.road_tile_posis) {
            return Err("All road tiles must be connected to each other");
        }
        if !Self::are_tiles_connected(&self.building_tile_posis) {
            return Err("All building tiles must be connected to each other");
        }
        if Self::have_tiles_more_than_max_neighbors(2, &self.road_tile_posis) {
            return Err("Only one clear road allowed");
        }
        if self.road_end_pos().is_none() {
            return Err("Road must end surrounded by three building tiles");
        }

        Ok(())
    }

    fn check_neighbors(pos: UVec2, tiles: &HashSet<UVec2>, linked: &mut HashSet<UVec2>) {
        linked.insert(pos);
        for neighbor in Self::get_neighbors(&pos, &tiles) {
            if tiles.get(&neighbor).is_some() && linked.get(&neighbor).is_none() {
                Self::check_neighbors(neighbor, tiles, linked);
            }
        }
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
            Self::check_neighbors(start.clone(), tiles, &mut connected_tiles);
        }
        tiles.len() == connected_tiles.len()
    }
}
