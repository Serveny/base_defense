use super::step::BoardStep;
use crate::{board::step::BoardDirection, utils::Vec2Board};
use bevy::prelude::UVec2;
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
pub struct SpawnLine {
    pub start: Vec2Board,
    pub end: Vec2Board,
    pub range: RangeInclusive<f32>,
}

impl Default for SpawnLine {
    fn default() -> Self {
        Self {
            start: Vec2Board::default(),
            end: Vec2Board::default(),
            range: (0.0..=0.0),
        }
    }
}

impl SpawnLine {
    pub fn new(start: Option<UVec2>, first_step: Option<&BoardStep>) -> Self {
        use BoardDirection::*;
        match (start, first_step) {
            (Some(start), Some(step)) => match step.direction {
                East | West => {
                    let start = Vec2Board::from(start);
                    let end = Vec2Board::new(start.x as f32, start.y as f32 + 1.);
                    Self {
                        start,
                        end,
                        range: start.y..=end.y,
                    }
                }
                North | South => {
                    let start = Vec2Board::from(start);
                    let end = Vec2Board::new(start.x as f32 + 1., start.y as f32);
                    Self {
                        start,
                        end: Vec2Board::new(start.x as f32, start.y as f32 + 1.),
                        range: start.x..=end.x,
                    }
                }
            },
            _ => Self::default(),
        }
    }

    #[allow(dead_code)]
    pub fn from_vecs(start: Vec2Board, end: Vec2Board) -> Self {
        match start.y == end.y {
            true => Self {
                start,
                end,
                range: start.x..=end.x,
            },
            false => Self {
                start,
                end,
                range: start.y..=end.y,
            },
        }
    }
}
