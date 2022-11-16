use super::GameScreen;
use crate::{
    board::{
        spawn_line::SpawnLine,
        step::{BoardDirection, BoardStep},
        visualisation::TILE_SIZE,
        BoardCache,
    },
    utils::{
        health_bar::health_bar, range_circle::RangeCircle, speed::Speed, TilesPerSecond, Vec2Board,
    },
};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use euclid::Angle;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, ops::RangeInclusive, time::Duration};

pub type IsRoadEnd = bool;

#[derive(Clone, Serialize, Deserialize)]
pub enum EnemyType {
    Normal,
    Speeder,
    Tank,
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub size_radius: f32,
    pub break_radius: f32,
    speed: TilesPerSecond,
    pub health_max: f32,
    pub health: f32,
    pub pos: Vec2Board,
    enemy_type: EnemyType,
    current_step: BoardStep,

    // Tower can reserve damage, so other towers will not shoot at this enemy if damage == health
    pub reserved_damage: f32,
    pub path_offset: f32,
    pub is_in_spawn: bool,
}

impl Enemy {
    pub fn new(
        enemy_type: EnemyType,
        q_enemies: &Query<&Enemy>,
        board_cache: &BoardCache,
    ) -> Option<Self> {
        let first_step = board_cache.road_path.first().unwrap().clone();
        match enemy_type {
            EnemyType::Normal => Self::new_normal(first_step, q_enemies, board_cache),
            EnemyType::Speeder => Self::new_speeder(first_step, q_enemies, board_cache),
            EnemyType::Tank => Self::new_tank(first_step, q_enemies, board_cache),
        }
    }

    fn generate_offset(
        size_radius: f32,
        q_enemies: &Query<&Enemy>,
        board_cache: &BoardCache,
    ) -> Option<f32> {
        let ranges = Self::find_free_ranges(q_enemies, size_radius, &board_cache.spawn_line);
        if ranges.is_empty() {
            None
        } else {
            let range = ranges[rand::thread_rng().gen_range(0..ranges.len())].clone();
            Some(rand::thread_rng().gen_range(range) - 0.5 - *board_cache.spawn_line.range.start())
        }
    }

    pub fn new_normal(
        mut current_step: BoardStep,
        q_enemies: &Query<&Enemy>,
        board_cache: &BoardCache,
    ) -> Option<Self> {
        let size_radius = 0.125;
        if let Some(path_offset) = Self::generate_offset(size_radius, q_enemies, board_cache) {
            current_step.distance += 0.5 - path_offset;
            let speed = 1.;
            return Some(Self {
                size_radius,
                break_radius: size_radius + (size_radius / 10.),
                speed,
                health_max: 100.,
                health: 100.,
                pos: first_pos(&current_step, path_offset),
                enemy_type: EnemyType::Normal,
                current_step,
                reserved_damage: 0.,
                path_offset,
                is_in_spawn: true,
            });
        }
        None
    }

    pub fn new_speeder(
        mut current_step: BoardStep,
        q_enemies: &Query<&Enemy>,
        board_cache: &BoardCache,
    ) -> Option<Self> {
        let size_radius = 0.075;
        if let Some(path_offset) = Self::generate_offset(size_radius, q_enemies, board_cache) {
            current_step.distance += 0.5 - path_offset;
            let speed = 2.;
            return Some(Self {
                size_radius,
                break_radius: size_radius + (size_radius / 10.),
                speed,
                health_max: 10.,
                health: 10.,
                pos: first_pos(&current_step, path_offset),
                enemy_type: EnemyType::Speeder,
                current_step,
                reserved_damage: 0.,
                path_offset,
                is_in_spawn: true,
            });
        }
        None
    }

    pub fn new_tank(
        mut current_step: BoardStep,
        q_enemies: &Query<&Enemy>,
        board_cache: &BoardCache,
    ) -> Option<Self> {
        let size_radius = 0.25;
        if let Some(path_offset) = Self::generate_offset(size_radius, q_enemies, board_cache) {
            current_step.distance += 0.5 - path_offset;
            let speed = 0.5;
            return Some(Self {
                size_radius,
                break_radius: size_radius + (size_radius / 10.),
                speed,
                health_max: 1000.,
                health: 1000.,
                pos: first_pos(&current_step, path_offset),
                enemy_type: EnemyType::Tank,
                current_step,
                reserved_damage: 0.,
                path_offset,
                is_in_spawn: true,
            });
        }
        None
    }

    #[allow(dead_code)]
    pub fn new_dummy(pos: Vec2Board) -> Self {
        Self {
            size_radius: 1.,
            break_radius: 2.,
            speed: 1.,
            health_max: 100.,
            health: 100.,
            pos,
            enemy_type: EnemyType::Normal,
            current_step: BoardStep::default(),
            reserved_damage: 0.,
            path_offset: 0.,
            is_in_spawn: false,
        }
    }

    pub fn distance_walked(speed: f32, dur: Duration) -> f32 {
        speed * dur.as_secs_f32()
    }

    // Return true if end is reached
    pub fn walk_until_end(
        &mut self,
        dur: Duration,
        speed: TilesPerSecond,
        board_cache: &BoardCache,
    ) -> IsRoadEnd {
        match self.current_step.is_end_reached() {
            true => self.step_end_reached(board_cache),
            false => self.walk(dur, speed),
        }
    }

    fn step_end_reached(&mut self, board_cache: &BoardCache) -> IsRoadEnd {
        let step = &mut self.current_step;
        if let Some(next) = next_step(&board_cache.road_path, step, self.path_offset) {
            *step = next;
            return false;
        }
        true
    }

    fn walk(&mut self, dur: Duration, speed: TilesPerSecond) -> IsRoadEnd {
        let mut step = &mut self.current_step;
        let dist = Self::distance_walked(speed, dur);
        step.distance_walked += dist;
        self.pos.add_in_direction(dist, step.direction);
        false
    }

    pub fn is_in_range(&self, tower_pos: Vec2Board, range_radius: f32) -> bool {
        self.pos.distance(tower_pos.into()) <= range_radius
    }

    pub fn health_as_percent(&self) -> f32 {
        self.health / self.health_max
    }

    pub fn spawn(self, cmds: &mut Commands) {
        match self.enemy_type {
            EnemyType::Normal => spawn_normal_enemy(cmds, self),
            EnemyType::Speeder => todo!(),
            EnemyType::Tank => spawn_tank_enemy(cmds, self),
        }
    }

    fn find_free_ranges(
        q_enemies: &Query<&Enemy>,
        mut new_enemy_radius: f32,
        spawn_line: &SpawnLine,
    ) -> Vec<RangeInclusive<f32>> {
        new_enemy_radius *= 2.;
        let start_range = (spawn_line.range.start() + new_enemy_radius)
            ..=(spawn_line.range.end() - new_enemy_radius);
        let enemies: Vec<&Enemy> = q_enemies.iter().filter(|enemy| enemy.is_in_spawn).collect();
        if enemies.is_empty() {
            vec![start_range]
        } else {
            find_gaps(
                start_range.clone(),
                enemies
                    .iter()
                    .filter_map(|enemy| {
                        RangeCircle::new(enemy.pos, enemy.size_radius + new_enemy_radius)
                            .intersection_range(spawn_line)
                    })
                    .filter_map(|range| Self::set_range_to_padding(range, &start_range))
                    .collect(),
            )
            .into_iter()
            .filter(|range| range.end() - range.start() > new_enemy_radius)
            .collect()
        }
    }

    fn set_range_to_padding(
        range: RangeInclusive<f32>,
        start_range: &RangeInclusive<f32>,
    ) -> Option<RangeInclusive<f32>> {
        if start_range.end() < range.start() || start_range.start() > range.end() {
            return None;
        }
        let start = start_range.start().clamp(*range.start(), *range.end());
        let end = start_range.end().clamp(*range.start(), *range.end());
        Some(start..=end)
    }

    pub fn is_behind_of(&self, other: &Enemy) -> bool {
        let s_step = &self.current_step;
        let o_step = &other.current_step;

        o_step.road_path_index > s_step.road_path_index
            || (o_step.road_path_index == s_step.road_path_index
                && self.is_behind_of_in_step(other))
    }

    fn is_behind_of_in_step(&self, other: &Enemy) -> bool {
        use BoardDirection::*;
        match self.current_step.direction {
            East => other.pos.x > self.pos.x,
            North => other.pos.y > self.pos.y,
            West => other.pos.x < self.pos.x,
            South => other.pos.y < self.pos.y,
        }
    }
}

fn find_gaps(
    main_range: RangeInclusive<f32>,
    mut ranges: Vec<RangeInclusive<f32>>,
) -> Vec<RangeInclusive<f32>> {
    ranges = merge_ranges(ranges);
    let mut gaps = vec![*main_range.start()..=*ranges.first().unwrap().start()];
    ranges.windows(2).for_each(|two_ranges| {
        gaps.push(*two_ranges[0].end()..=*two_ranges[1].start());
    });
    gaps.push(*ranges.last().unwrap().end()..=*main_range.end());
    gaps
}

fn sort_by_start_then_by_end(a: &RangeInclusive<f32>, b: &RangeInclusive<f32>) -> Ordering {
    let comp = a.start().total_cmp(b.start());
    match comp {
        Ordering::Equal => a.end().total_cmp(b.end()),
        _ => comp,
    }
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<f32>>) -> Vec<RangeInclusive<f32>> {
    if ranges.is_empty() {
        return ranges;
    }
    ranges.sort_by(sort_by_start_then_by_end);
    let mut merged = Vec::new();
    let mut current = ranges.first().unwrap().clone();

    ranges[1..].iter().for_each(|other| {
        if *current.end() < *other.start() {
            merged.push(current.clone());
            current = other.clone();
        } else {
            current = *current.start()..=(*other.end()).max(*current.end());
        }
    });
    merged.push(current);
    merged
}

#[cfg(test)]
mod gaps_tests {
    use super::find_gaps;

    #[test]
    fn test_find_gaps_sorted() {
        let main_range = 0.0..=10.0;
        let ranges = vec![1.0..=2.0, 4.0..=6.0];
        assert_eq!(
            find_gaps(main_range, ranges),
            vec![0.0..=1.0, 2.0..=4.0, 6.0..=10.0]
        )
    }

    #[test]
    fn test_find_gaps_unsorted() {
        let main_range = 0.0..=10.0;
        let ranges = vec![4.0..=6.0, 1.0..=2.0];
        assert_eq!(
            find_gaps(main_range, ranges),
            vec![0.0..=1.0, 2.0..=4.0, 6.0..=10.0]
        )
    }

    #[test]
    fn test_find_gaps_unsorted_overlapped() {
        let main_range = 0.0..=10.0;
        let ranges = vec![4.0..=6.0, 4.0..=5.0, 1.0..=2.0];
        assert_eq!(
            find_gaps(main_range, ranges),
            vec![0.0..=1.0, 2.0..=4.0, 6.0..=10.0]
        )
    }

    #[test]
    fn test_find_gaps_range_in_range() {
        let main_range = 0.0..=10.0;
        let ranges = vec![4.0..=6.0, 4.5..=5.0, 4.6..=4.9, 1.0..=2.0];
        assert_eq!(
            find_gaps(main_range, ranges),
            vec![0.0..=1.0, 2.0..=4.0, 6.0..=10.0]
        )
    }
}

#[cfg(test)]
mod enemy_tests {
    use crate::utils::Vec2Board;

    use super::Enemy;

    #[test]
    fn test_set_range_to_padding_range_inside() {
        let start_range = 0.0..=4.0;
        let range = 1.0..=3.0;
        assert_eq!(
            Enemy::set_range_to_padding(range.clone(), &start_range),
            Some(range)
        );
    }

    #[test]
    fn test_set_range_to_padding_start_outside() {
        let start_range = 1.0..=3.0;
        let range = 0.0..=2.0;
        assert_eq!(
            Enemy::set_range_to_padding(range, &start_range),
            Some(1.0..=2.0)
        );
    }

    #[test]
    fn test_set_range_to_padding_end_outside() {
        let start_range = 0.0..=2.0;
        let range = 0.0..=3.0;
        assert_eq!(
            Enemy::set_range_to_padding(range, &start_range),
            Some(0.0..=2.0)
        );
    }

    #[test]
    fn test_set_range_to_padding_start_end_outside() {
        let start_range = 1.0..=2.0;
        let range = 1.0..=3.0;
        assert_eq!(
            Enemy::set_range_to_padding(range, &start_range),
            Some(1.0..=2.0)
        );
    }

    #[test]
    fn test_set_range_to_padding_dec() {
        let start_range = 1.0..=2.0;
        let range = 0.69077474..=2.4369025;
        assert_eq!(
            Enemy::set_range_to_padding(range, &start_range),
            Some(start_range)
        );
    }

    #[test]
    fn test_is_in_front() {
        let enemy_1 = Enemy::new_dummy(Vec2Board::new(1., 0.));
        let mut enemy_2 = Enemy::new_dummy(Vec2Board::new(2., 0.));
        enemy_2.current_step.distance_walked = 1.;
        assert!(enemy_1.is_behind_of(&enemy_2));
    }

    #[test]
    fn test_is_not_in_front() {
        let enemy_1 = Enemy::new_dummy(Vec2Board::new(1., 0.));
        let mut enemy_2 = Enemy::new_dummy(Vec2Board::new(4., 0.));
        enemy_2.current_step.distance_walked = 4.;
        assert!(enemy_1.is_behind_of(&enemy_2));
    }
}
pub(super) fn spawn_normal_enemy(cmds: &mut Commands, enemy: Enemy) {
    cmds.spawn(enemy_normal_shape(&enemy))
        .with_children(|parent| {
            health_bar(parent, TILE_SIZE / 8.);
        })
        .insert(Speed::new(enemy.speed))
        .insert(enemy)
        .insert(GameScreen);
}

fn enemy_normal_shape(enemy: &Enemy) -> ShapeBundle {
    let line_width = TILE_SIZE / 24.;
    let shape = shapes::RegularPolygon {
        sides: 5,
        feature: shapes::RegularPolygonFeature::Radius(
            enemy.size_radius * TILE_SIZE - (line_width / 2.),
        ),
        ..shapes::RegularPolygon::default()
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::MAROON),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, line_width),
        },
        Transform {
            translation: enemy.pos.to_scaled_vec3(1.),
            ..Default::default()
        },
    )
}

pub(super) fn spawn_tank_enemy(cmds: &mut Commands, enemy: Enemy) {
    cmds.spawn(enemy_tank_shape(&enemy))
        .with_children(|parent| {
            health_bar(parent, enemy.size_radius * TILE_SIZE);
        })
        .insert(Speed::new(enemy.speed))
        .insert(enemy)
        .insert(GameScreen);
}

fn enemy_tank_shape(enemy: &Enemy) -> ShapeBundle {
    let line_width = TILE_SIZE / 24.;
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(
            enemy.size_radius * TILE_SIZE - (line_width / 2.),
        ),
        ..shapes::RegularPolygon::default()
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::OLIVE),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, line_width),
        },
        Transform {
            translation: enemy.pos.to_scaled_vec3(1.),
            ..Default::default()
        },
    )
}
pub fn next_step(path: &[BoardStep], last: &BoardStep, offset: f32) -> Option<BoardStep> {
    if let Some(next) = path.get(last.road_path_index + 1) {
        let mut new_step = next.clone();
        new_step.start_pos = last.end_pos();
        new_step.distance += next_offset(last, next, path.get(next.road_path_index + 1), offset);
        return Some(new_step);
    }
    None
}

fn first_pos(first_step: &BoardStep, offset: f32) -> Vec2Board {
    let pos = first_step.start_pos;
    use crate::board::step::BoardDirection::*;
    match first_step.direction {
        East => Vec2Board::new(pos.x - 0.5, pos.y + offset),
        North => Vec2Board::new(pos.x + offset, pos.y - 0.5),
        West => Vec2Board::new(pos.x + 0.5, pos.y + offset),
        South => Vec2Board::new(pos.x + offset, pos.y + 0.5),
    }
}

fn next_offset(
    last: &BoardStep,
    next: &BoardStep,
    overnext: Option<&BoardStep>,
    offset: f32,
) -> f32 {
    relative_multiplier(last, next)
        * if let Some(overnext) = overnext {
            match last.direction == overnext.direction {
                true => 0.,
                false => offset * 2.,
            }
        } else {
            offset
        }
}

fn relative_multiplier(last: &BoardStep, next: &BoardStep) -> f32 {
    let last_vec = last.direction.as_vec2board();
    let next_vec = next.direction.as_vec2board();
    let angle = Angle::radians(last_vec.angle_between(next_vec.into())).to_degrees();
    angle / -90.
}
