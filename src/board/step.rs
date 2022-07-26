use crate::utils::Vec2Board;
use euclid::Angle;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BoardDirection {
    East,
    North,
    West,
    South,
}

impl Default for BoardDirection {
    fn default() -> Self {
        Self::East
    }
}

impl BoardDirection {
    pub fn inverted(self) -> Self {
        use BoardDirection::*;
        match self {
            East => West,
            North => South,
            West => East,
            South => North,
        }
    }
    pub fn as_vec2board(&self) -> Vec2Board {
        use BoardDirection::*;
        match self {
            East => Vec2Board::new(1., 0.),
            North => Vec2Board::new(0., 1.),
            West => Vec2Board::new(-1., 0.),
            South => Vec2Board::new(0., -1.),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BoardStep {
    pub road_path_index: usize,
    pub direction: BoardDirection,
    pub distance: f32,
    pub distance_walked: f32,
    pub start_pos: Vec2Board,
}

// Constructors
impl BoardStep {
    pub fn new(road_path_index: usize, path: Vec2Board, start_pos: Vec2Board) -> Self {
        Self {
            road_path_index,
            direction: Self::direction(path),
            distance: path.distance_from_zero(),
            distance_walked: 0.,
            start_pos,
        }
    }

    pub fn end_pos(&self) -> Vec2Board {
        (*self.start_pos + (*self.direction.as_vec2board() * self.distance)).into()
    }
}

impl BoardStep {
    pub fn is_end_reached(&self) -> bool {
        // println!("{} {}", self.distance_walked, self.distance);
        self.distance_walked >= self.distance
    }

    pub fn angle(&self) -> Angle<f32> {
        Angle::degrees(match self.direction {
            BoardDirection::West => 0.,
            BoardDirection::North => 270.,
            BoardDirection::East => 180.,
            BoardDirection::South => 90.,
        })
    }

    fn direction(path: Vec2Board) -> BoardDirection {
        if path.x > 0. {
            BoardDirection::East
        } else if path.y > 0. {
            BoardDirection::North
        } else if path.x < 0. {
            BoardDirection::West
        } else {
            BoardDirection::South
        }
    }
}
