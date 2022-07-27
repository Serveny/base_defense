use super::Vec2Board;
use bevy::prelude::*;

pub struct Circle {
    pub middle: Vec2Board,
    pub radius: f32,
}

impl Circle {
    pub fn new(middle: Vec2Board, radius: f32) -> Self {
        Self { middle, radius }
    }

    fn is_inside(
        &self,
        vec_start: Vec2Board,
        vec_end: Vec2Board,
    ) -> Option<(f32, Vec2Board, Vec2Board)> {
        let ac = self.middle - vec_start;
        let ab = vec_end - vec_start;
        let l = ac.project_onto(*ab);
        let p = vec_start + l.into();
        let lot_length = self.middle.distance(*p);
        println!(
            "ac: {}, ab: {}, l: {}, p: {}, lot_length: {}",
            ac, ab, l, p, lot_length
        );
        if lot_length < self.radius {
            return Some((lot_length, p, ab));
        }
        None
    }

    pub fn target_point(&self, vec_start: Vec2, vec_end: Vec2) -> Option<Vec2> {
        let middle = *self.middle;
        let ac = middle - vec_start;

        let ab = vec_end - vec_start;
        let l = ac.project_onto(ab);
        let p = vec_start + l;
        let lot_len = middle.distance(p);
        let dis_start = middle.distance(vec_start);
        let dis_end = middle.distance(vec_end);
        let s1p = (self.radius.powi(2) - lot_len.powi(2)).sqrt();
        let s1 = p - (s1p * ab.normalize());

        let is_a = dis_start <= self.radius && dis_end <= self.radius;
        let is_b = dis_start <= self.radius && dis_end >= self.radius;
        let is_c = dis_start > self.radius && dis_end < self.radius;
        let is_d = lot_len != self.radius && dis_start > self.radius && dis_end > self.radius;
        let is_e = lot_len == self.radius;
        let is_f = lot_len > self.radius;

        return if is_f {
            None
        } else {
            Some(if is_a || is_b {
                vec_start
            } else if is_c || is_d {
                s1
            } else if is_e {
                p
            } else {
                panic!("Nope")
            })
        };
    }

    fn is_vec2_in(vec_check: Vec2, vec_start: Vec2Board, vec_end: Vec2Board) -> bool {
        Self::epsilon_eq(
            vec_start.distance(vec_check) + vec_end.distance(vec_check),
            vec_start.distance(*vec_end),
            0.0001,
        )
    }

    fn epsilon_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() < eps
    }

    pub fn intersections_inside(
        &self,
        vec_start: Vec2Board,
        vec_end: Vec2Board,
    ) -> Option<Vec<Vec2Board>> {
        if let Some((lot_lenght, p, ab)) = self.is_inside(vec_start, vec_end) {
            let mut vec = Vec::new();
            let s1p = (self.radius.powi(2) - lot_lenght.powi(2)).sqrt();
            let multi: f32 = (ab.x + ab.y).signum();
            let s1 = Vec2Board::new(p.x - s1p * multi, p.y - s1p * multi).project_onto(*ab);
            let s2 = Vec2Board::new(p.x + s1p * multi, p.y + s1p * multi).project_onto(*ab);
            if Self::is_vec2_in(s1, vec_start, vec_end) {
                vec.push(s1.into());
            }
            if Self::is_vec2_in(s2, vec_start, vec_end) {
                vec.push(s2.into());
            }
            if vec.is_empty() {
                vec.push(vec_start);
                vec.push(vec_end);
            }
            return Some(vec);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;

    use super::Circle;
    use crate::utils::Vec2Board;

    #[test]
    fn test_circle_not_inside() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 1.);
        let vec_start = Vec2Board::new(-2., 2.);
        let vec_end = Vec2Board::new(2., 2.);
        assert_eq!(circle.is_inside(vec_start, vec_end), None);
    }

    #[test]
    fn test_circle_inside() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 4.);
        let vec_start = Vec2Board::new(-2., 0.);
        let vec_end = Vec2Board::new(2., 0.);
        assert_ne!(circle.is_inside(vec_start, vec_end), None);
    }

    #[test]
    fn test_circle_inside_2() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 1.);
        let vec_start = Vec2Board::new(-2., 0.);
        let vec_end = Vec2Board::new(2., 0.);
        assert_ne!(circle.is_inside(vec_start, vec_end), None);
    }

    #[test]
    fn test_circle_intersections() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 1.);
        let vec_start = Vec2Board::new(2., 0.);
        let vec_end = Vec2Board::new(-2., 0.);
        assert_eq!(
            circle.intersections_inside(vec_start, vec_end),
            Some(vec![Vec2Board::new(1., 0.), Vec2Board::new(-1., 0.)])
        );
    }

    #[test]
    fn test_circle_intersections_2() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 1.);
        let vec_start = Vec2Board::new(-2., 0.);
        let vec_end = Vec2Board::new(2., 0.);
        assert_eq!(
            circle.intersections_inside(vec_start, vec_end),
            Some(vec![Vec2Board::new(-1., 0.), Vec2Board::new(1., 0.)]),
        );
    }

    #[test]
    fn test_circle_one_intersection() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 1.);
        let vec_start = Vec2Board::new(-2., 0.);
        let vec_end = Vec2Board::new(0., 0.);
        assert_eq!(
            circle.intersections_inside(vec_start, vec_end),
            Some(vec![Vec2Board::new(-1., 0.)]),
        );
    }

    #[test]
    fn test_circle_both_inside() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 2.);
        let vec_start = Vec2Board::new(-1., 0.);
        let vec_end = Vec2Board::new(1., 0.);
        assert_eq!(
            circle.intersections_inside(vec_start, vec_end),
            Some(vec![Vec2Board::new(-1., 0.), Vec2Board::new(1., 0.)])
        );
    }

    #[test]
    fn test_target_a() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-1., 0.);
        let end = Vec2::new(1., 0.);

        assert_eq!(circle.target_point(start, end), Some(start));
    }

    #[test]
    fn test_target_b() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-1., 1.);
        let end = Vec2::new(3., 1.);

        assert_eq!(circle.target_point(start, end), Some(start));
    }

    #[test]
    fn test_target_c() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 0.);
        let end = Vec2::new(1., 0.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(-2., 0.)));
    }

    #[test]
    fn test_target_d() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 0.);
        let end = Vec2::new(3., 0.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(-2., 0.)));
    }

    #[test]
    fn test_target_e() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 2.);
        let start = Vec2::new(-3., 2.);
        let end = Vec2::new(3., 2.);

        assert_eq!(circle.target_point(start, end), Some(Vec2::new(0., 2.)));
    }

    #[test]
    fn test_target_f() {
        let circle = Circle::new(Vec2Board::new(0., 0.), 1.);
        let start = Vec2::new(-3., 2.);
        let end = Vec2::new(3., 2.);

        assert_eq!(circle.target_point(start, end), None);
    }
}
