use super::Vec2Board;
use bevy::prelude::*;

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
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use super::RangeCircle;
    use crate::utils::Vec2Board;

    #[test]
    fn test_target_a() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-1., 0.);
        let end = Vec2::new(1., 0.);

        assert_eq!(circle.target_point(start, end), Some(start));
    }

    #[test]
    fn test_target_b() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-1., 1.);
        let end = Vec2::new(3., 1.);

        assert_eq!(circle.target_point(start, end), Some(start));
    }

    #[test]
    fn test_target_c() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 0.);
        let end = Vec2::new(1., 0.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(-2., 0.)));
    }

    #[test]
    fn test_target_d() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 0.);
        let end = Vec2::new(3., 0.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(-2., 0.)));
    }

    #[test]
    fn test_target_e() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 2.);
        let end = Vec2::new(3., 2.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(0., 2.)));
    }

    #[test]
    fn test_target_f() {
        let circle = RangeCircle::new(Vec2Board::new(0., 0.), 1.);
        let start = Vec2::new(-3., 2.);
        let end = Vec2::new(3., 2.);

        assert_eq!(circle.target_point(start, end), None);
    }
}
