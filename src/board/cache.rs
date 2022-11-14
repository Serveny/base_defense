use super::{spawn_line::SpawnLine, step::BoardStep, Board, Tile};
use crate::utils::Vec2Board;
use bevy::prelude::*;
use indexmap::IndexSet;

const TILE_NEIGHBOR_MATRIX: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(Resource, Clone)]
pub struct BoardCache {
    pub tower_tile_posis: IndexSet<UVec2>,
    pub building_tile_posis: IndexSet<UVec2>,
    pub road_tile_posis: IndexSet<UVec2>,
    pub road_start_pos: Option<UVec2>,
    pub road_end_pos: Option<UVec2>,
    pub road_path: Vec<BoardStep>,
    pub spawn_line: SpawnLine,
}

// Static functions
impl BoardCache {
    pub fn new(board: &Board) -> Self {
        let tower_tile_posis = board.get_tiles(Tile::TowerGround);
        let building_tile_posis = board.get_tiles(Tile::BuildingGround);
        let mut road_tile_posis = board.get_tiles(Tile::Road);
        let road_start = Self::road_start_pos_from(&road_tile_posis, board.width, board.height);
        let road_end = Self::road_end_pos_from(&road_tile_posis, &building_tile_posis);
        let mut road_path = Vec::new();
        if let Some(road_start) = road_start {
            if let Some(posis) = Self::are_tiles_connected(&road_tile_posis, road_start) {
                road_tile_posis = posis;
                road_path = Self::calc_road_path(&road_tile_posis);
            }
        }
        Self {
            spawn_line: SpawnLine::new(road_start, road_path.first()),
            tower_tile_posis,
            building_tile_posis,
            road_tile_posis,
            road_start_pos: road_start,
            road_end_pos: road_end,
            road_path,
        }
    }

    // is tile at one edge of the board and has only one road neighbor, it is the starting point
    fn road_start_pos_from(
        road_tile_posis: &IndexSet<UVec2>,
        board_width: u8,
        board_heigt: u8,
    ) -> Option<UVec2> {
        for pos in road_tile_posis {
            let (x, y) = (pos.x, pos.y);
            if (x == 0 || y == 0 || x == board_width as u32 - 1 || y == board_heigt as u32 - 1)
                && Self::get_neighbors(pos, road_tile_posis).len() == 1
            {
                return Some(*pos);
            }
        }
        None
    }

    fn get_neighbors(pos: &UVec2, tiles: &IndexSet<UVec2>) -> Vec<UVec2> {
        let mut neighbors = Vec::new();
        for additor in TILE_NEIGHBOR_MATRIX {
            let x = pos.x as i32 + additor.0;
            let y = pos.y as i32 + additor.1;
            if x >= 0 && y >= 0 {
                let neighbor = UVec2::new(x as u32, y as u32);
                if tiles.contains(&neighbor) {
                    neighbors.push(neighbor);
                }
            }
        }
        neighbors
    }

    fn calc_road_path(rt_posis: &IndexSet<UVec2>) -> Vec<BoardStep> {
        let pos = Vec2Board::from_uvec2_middle(rt_posis.get_index(0).unwrap());
        let mut path = vec![BoardStep::new(
            0,
            Vec2Board::from_uvec2_middle(rt_posis.get_index(1).unwrap())
                - Vec2Board::from_uvec2_middle(rt_posis.get_index(0).unwrap()),
            pos,
        )];
        for (i, first_pos) in rt_posis.iter().enumerate() {
            match rt_posis.get_index(i + 2) {
                Some(third_pos) => {
                    match Self::next_step(rt_posis, i, first_pos, third_pos, path.len()) {
                        Some(step) => path.push(step),
                        None => path.last_mut().unwrap().distance += 1.,
                    }
                }
                None => break,
            }
        }
        path
    }

    fn next_step(
        rt_posis: &IndexSet<UVec2>,
        i: usize,
        first_pos: &UVec2,
        third_pos: &UVec2,
        path_i: usize,
    ) -> Option<BoardStep> {
        let second_pos = rt_posis.get_index(i + 1).unwrap();
        let this_vec =
            Vec2Board::from_uvec2_middle(second_pos) - Vec2Board::from_uvec2_middle(first_pos);
        let next_vec =
            Vec2Board::from_uvec2_middle(third_pos) - Vec2Board::from_uvec2_middle(second_pos);
        if this_vec != next_vec {
            return Some(BoardStep::new(
                path_i,
                next_vec,
                Vec2Board::from_uvec2_middle(second_pos),
            ));
        }
        None
    }
}

// Validation
impl BoardCache {
    pub fn validate(&self) -> Result<(), &str> {
        if self.tower_tile_posis.is_empty() {
            return Err("Need tower tiles");
        }
        if self.building_tile_posis.len() < 5 {
            return Err("Need minimal five building tiles");
        }
        if self.road_tile_posis.len() < 3 {
            return Err("Need minimal two road tiles");
        }
        if let Some(start) = self.road_start_pos {
            if Self::are_tiles_connected(&self.road_tile_posis, start).is_none() {
                return Err("All road tiles must be connected to each other");
            }
        } else {
            return Err("Need road starting point at the board edge");
        }
        let build_tile = self.building_tile_posis.first().unwrap();
        if Self::are_tiles_connected(&self.building_tile_posis, *build_tile).is_none() {
            return Err("All building tiles must be connected to each other");
        }
        if Self::have_tiles_more_than_max_neighbors(2, &self.road_tile_posis) {
            return Err("Only one clear road allowed");
        }
        if self.road_end_pos.is_none() {
            return Err("Road must end surrounded by three building tiles");
        }

        Ok(())
    }

    fn are_tiles_connected(tile_posis: &IndexSet<UVec2>, start: UVec2) -> Option<IndexSet<UVec2>> {
        let mut connected_tiles: IndexSet<UVec2> = IndexSet::new();
        Self::check_neighbors(start, tile_posis, &mut connected_tiles);
        if tile_posis.len() == connected_tiles.len() {
            Some(connected_tiles)
        } else {
            None
        }
    }

    fn check_neighbors(pos: UVec2, tiles: &IndexSet<UVec2>, linked: &mut IndexSet<UVec2>) {
        linked.insert(pos);
        for neighbor in Self::get_neighbors(&pos, tiles) {
            if tiles.get(&neighbor).is_some() && linked.get(&neighbor).is_none() {
                Self::check_neighbors(neighbor, tiles, linked);
            }
        }
    }

    fn have_tiles_more_than_max_neighbors(max: usize, tiles: &IndexSet<UVec2>) -> bool {
        for tile in tiles {
            if Self::get_neighbors(tile, tiles).len() > max {
                return true;
            }
        }
        false
    }
}

// editor functions
impl BoardCache {
    pub fn get_tile_posis_mut(&mut self, tile: &Tile) -> Option<&mut IndexSet<UVec2>> {
        match tile {
            Tile::TowerGround => Some(&mut self.tower_tile_posis),
            Tile::BuildingGround => Some(&mut self.building_tile_posis),
            Tile::Road => Some(&mut self.road_tile_posis),
            Tile::Empty => None,
        }
    }

    pub fn calc_road_data(&mut self, board: &Board) {
        self.road_start_pos =
            Self::road_start_pos_from(&self.road_tile_posis, board.width, board.height);
        self.road_end_pos =
            Self::road_end_pos_from(&self.road_tile_posis, &self.building_tile_posis);
    }

    pub fn remove_tile_pos(&mut self, pos: &UVec2, tile: &Tile) {
        if let Some(new_posis) = self.get_tile_posis_mut(tile) {
            new_posis.remove(pos);
        }
    }

    pub fn insert_tile_pos(&mut self, pos: UVec2, tile: &Tile) {
        if let Some(new_posis) = self.get_tile_posis_mut(tile) {
            new_posis.insert(pos);
        }
    }

    // if tile is surrounded by three building tiles, it is the ending point
    pub fn road_end_pos_from(
        road_tile_posis: &IndexSet<UVec2>,
        building_tile_posis: &IndexSet<UVec2>,
    ) -> Option<UVec2> {
        for pos in road_tile_posis {
            if Self::get_neighbors(pos, building_tile_posis).len() == 3 {
                return Some(*pos);
            }
        }
        None
    }
}
