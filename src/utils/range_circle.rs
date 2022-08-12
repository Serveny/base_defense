use crate::board::spawn_line::SpawnLine;

use super::Vec2Board;
use bevy::prelude::*;
use std::ops::RangeInclusive;

pub struct RangeCircle {
    pub middle: Vec2Board,
    pub radius: f32,
}

impl RangeCircle {
    pub fn new(middle: Vec2Board, radius: f32) -> Self {
        Self { middle, radius }
    }

    pub fn target_point(&self, vec_start: Vec2, vec_end: Vec2) -> Option<Vec2> {
        let middle = *self.middle;
        let ac = middle - vec_start;

        let ab = vec_end - vec_start;
        let l = ac.project_onto(ab);
        let p = vec_start + l;
        let lot_len = middle.distance(p);

        if lot_len <= self.radius {
            let dis_start = middle.distance(vec_start);
            let s1p = (self.radius.powi(2) - lot_len.powi(2)).sqrt();
            let s1 = p - (s1p * ab.normalize());

            Some(if dis_start <= self.radius {
                vec_start
            } else if lot_len == self.radius {
                p
            } else {
                s1
            })
        } else {
            None
        }
    }

    pub fn intersection_range(&self, spawn_line: &SpawnLine) -> Option<RangeInclusive<f32>> {
        let middle = *self.middle;
        let vec_start = *spawn_line.start;
        let vec_end = *spawn_line.end;
        let vec_range = &spawn_line.range;
        let ac = middle - vec_start;

        let ab = vec_end - vec_start;
        let l = ac.project_onto(ab);
        let p = vec_start + l;
        let lot_len = middle.distance(p);

        if lot_len == self.radius {
            // Special case because projected
            return Some(if vec_start.y == vec_end.y {
                p.y..=p.y
            } else {
                p.x..=p.x
            });
        } else if lot_len < self.radius {
            let s1p = (self.radius.powi(2) - lot_len.powi(2)).sqrt();
            let s1 = p - (s1p * ab.normalize());
            let s2 = p + (s1p * ab.normalize());

            return Some(if s1.x == s2.x {
                s1.y.max(*vec_range.start())..=s2.y.min(*vec_range.end())
            } else {
                s1.x.max(*vec_range.start())..=s2.x.min(*vec_range.end())
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::RangeCircle;
    use crate::{board::spawn_line::SpawnLine, utils::Vec2Board};
    use bevy::math::Vec2;

    #[test]
    fn test_target_vec_in_circle() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-1., 0.);
        let end = Vec2::new(1., 0.);

        assert_eq!(circle.target_point(start, end), Some(start));
    }

    #[test]
    fn test_target_start_in_end_out_circle() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-1., 1.);
        let end = Vec2::new(3., 1.);

        assert_eq!(circle.target_point(start, end), Some(start));
    }

    #[test]
    fn test_target_start_out_end_in_circle() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 0.);
        let end = Vec2::new(1., 0.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(-2., 0.)));
    }

    #[test]
    fn test_target_start_and_end_out_of_circle() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 0.);
        let end = Vec2::new(3., 0.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(-2., 0.)));
    }

    #[test]
    fn test_target_vec_on_circle() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 2.);
        let end = Vec2::new(3., 2.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(0., 2.)));
    }

    #[test]
    fn test_target_vec_out_of_circle() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 1.);
        let start = Vec2::new(-3., 2.);
        let end = Vec2::new(3., 2.);

        assert_eq!(circle.target_point(start, end), None);
    }

    #[test]
    fn test_intersection_range_start_and_end_out_of_circle() {
        let spawn_line = SpawnLine::from_vecs(Vec2Board::new(-3., 0.), Vec2Board::new(3., 0.));
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        assert_eq!(circle.intersection_range(&spawn_line), Some(-2.0..=2.0));
    }

    #[test]
    fn test_intersection_range_vec_out_of_circle() {
        let spawn_line = SpawnLine::from_vecs(Vec2Board::new(-3., 2.), Vec2Board::new(3., 2.));
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 1.);
        assert_eq!(circle.intersection_range(&spawn_line), None);
    }

    #[test]
    fn test_intersection_range_vec_on_circle() {
        let spawn_line = SpawnLine::from_vecs(Vec2Board::new(-3., 2.), Vec2Board::new(3., 2.));
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        assert_eq!(circle.intersection_range(&spawn_line), Some(2.0..=2.0));
    }

    #[test]
    fn test_intersection_range_vec_in_circle() {
        let spawn_line = SpawnLine::from_vecs(Vec2Board::new(0., 0.), Vec2Board::new(0., 1.));
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        assert_eq!(
            circle.intersection_range(&spawn_line),
            Some(spawn_line.range)
        );
    }
}
