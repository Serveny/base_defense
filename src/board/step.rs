use euclid::Angle;
use serde::{Deserialize, Serialize};

use crate::utils::Vec2Board;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BoardDirection {
    Right,
    Up,
    Left,
    Down,
}

impl Default for BoardDirection {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BoardStep {
    pub road_path_index: usize,
    pub direction: BoardDirection,
    pub distance: f32,
    pub distance_walked: f32,
}

// Constructors
impl BoardStep {
    pub fn new(road_path_index: usize, path: Vec2Board) -> Self {
        Self {
            road_path_index,
            direction: Self::direction(path),
            distance: path.distance_from_zero(),
            distance_walked: 0.,
        }
    }
}

impl BoardStep {
    pub fn is_end_reached(&self) -> bool {
        self.distance_walked >= self.distance
    }

    pub fn angle(&self) -> Angle<f32> {
        Angle::degrees(match self.direction {
            BoardDirection::Left => 0.,
            BoardDirection::Up => 90.,
            BoardDirection::Right => 180.,
            BoardDirection::Down => 270.,
        })
    }

    fn direction(path: Vec2Board) -> BoardDirection {
        if path.x > 0. {
            BoardDirection::Right
        } else if path.y > 0. {
            BoardDirection::Down
        } else if path.x < 0. {
            BoardDirection::Left
        } else {
            BoardDirection::Up
        }
    }
}
