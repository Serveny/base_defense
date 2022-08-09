use super::GameScreen;
use crate::{
    board::{step::BoardStep, visualisation::TILE_SIZE, BoardCache},
    utils::{health_bar::health_bar, TilesPerSecond, Vec2Board},
};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use euclid::Angle;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
    speed: TilesPerSecond,
    pub health_max: f32,
    pub health: f32,
    pub pos: Vec2Board,
    enemy_type: EnemyType,
    current_step: BoardStep,

    // Tower can reserve damage, so other towers will not shoot at this enemy if damage == health
    pub reserved_damage: f32,
    pub path_offset: f32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, board_cache: &BoardCache) -> Self {
        let first_step = board_cache.road_path.first().unwrap().clone();
        match enemy_type {
            EnemyType::Normal => Self::new_normal(first_step),
            EnemyType::Speeder => Self::new_speeder(first_step),
            EnemyType::Tank => Self::new_tank(first_step),
        }
    }

    fn generate_offset(size_radius: f32) -> f32 {
        let offset_max = 0.5 - size_radius;
        rand::thread_rng().gen_range((-offset_max)..(offset_max))
    }

    pub fn new_normal(mut current_step: BoardStep) -> Self {
        let size_radius = 0.125;
        let path_offset = Self::generate_offset(size_radius);
        current_step.distance += 0.5 - path_offset;
        Self {
            size_radius,
            speed: 1.,
            health_max: 100.,
            health: 100.,
            pos: first_pos(&current_step, path_offset),
            enemy_type: EnemyType::Normal,
            current_step,
            reserved_damage: 0.,
            path_offset,
        }
    }

    pub fn new_speeder(mut current_step: BoardStep) -> Self {
        let size_radius = 0.075;
        let path_offset = Self::generate_offset(size_radius);
        current_step.distance += 0.5 - path_offset;
        Self {
            size_radius,
            speed: 2.,
            health_max: 10.,
            health: 10.,
            pos: first_pos(&current_step, path_offset),
            enemy_type: EnemyType::Speeder,
            current_step,
            reserved_damage: 0.,
            path_offset,
        }
    }

    pub fn new_tank(mut current_step: BoardStep) -> Self {
        let size_radius = 0.25;
        let path_offset = Self::generate_offset(size_radius);
        current_step.distance += 0.5 - path_offset;
        Self {
            size_radius,
            speed: 0.5,
            health_max: 1000.,
            health: 1000.,
            pos: first_pos(&current_step, path_offset),
            enemy_type: EnemyType::Tank,
            current_step,
            reserved_damage: 0.,
            path_offset,
        }
    }

    pub fn distance_walked(speed: f32, dur: Duration) -> f32 {
        speed * dur.as_secs_f32()
    }

    // Return true if end is reached
    pub fn walk_until_end(&mut self, dur: Duration, board_cache: &BoardCache) -> IsRoadEnd {
        match self.current_step.is_end_reached() {
            true => self.step_end_reached(board_cache),
            false => self.walk(dur),
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

    fn walk(&mut self, dur: Duration) -> IsRoadEnd {
        let mut step = &mut self.current_step;
        let dist = Self::distance_walked(self.speed, dur);
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
}

pub(super) fn spawn_normal_enemy(cmds: &mut Commands, enemy: Enemy) {
    cmds.spawn_bundle(enemy_normal_shape(&enemy))
        .with_children(|parent| {
            health_bar(parent, TILE_SIZE / 8.);
        })
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
    cmds.spawn_bundle(enemy_tank_shape(&enemy))
        .with_children(|parent| {
            health_bar(parent, enemy.size_radius * TILE_SIZE);
        })
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
