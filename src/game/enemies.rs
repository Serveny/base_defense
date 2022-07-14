use super::{BoardVisu, GameScreen};
use crate::{
    board::{step::BoardStep, visualisation::TILE_SIZE, BoardCache},
    utils::{health_bar::health_bar, TilesPerSecond, Vec2Board},
};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Serialize, Deserialize)]
pub enum EnemyType {
    Normal,
    Speeder,
    Tank,
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct Enemy {
    speed: TilesPerSecond,
    pub health_max: f32,
    pub health: f32,
    pub pos: Vec2Board,
    enemy_type: EnemyType,
    current_step: BoardStep,

    // Tower can reserve damage, so other towers will not shoot at this enemy if damage == health
    pub reserved_damage: f32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, board_cache: &BoardCache) -> Self {
        let start_pos = Vec2Board::from_uvec2_middle(&board_cache.road_start_pos.unwrap());
        let first_step = board_cache.road_path.first().unwrap().clone();
        match enemy_type {
            EnemyType::Normal => Self::new_normal(start_pos, first_step),
            EnemyType::Speeder => Self::new_speeder(start_pos, first_step),
            EnemyType::Tank => Self::new_tank(start_pos, first_step),
        }
    }

    pub fn new_normal(pos: Vec2Board, current_step: BoardStep) -> Self {
        Self {
            speed: 1.,
            health_max: 100.,
            health: 100.,
            pos,
            enemy_type: EnemyType::Normal,
            current_step,
            reserved_damage: 0.,
        }
    }

    pub fn new_speeder(pos: Vec2Board, current_step: BoardStep) -> Self {
        Self {
            speed: 2.,
            health_max: 10.,
            health: 10.,
            pos,
            enemy_type: EnemyType::Speeder,
            current_step,
            reserved_damage: 0.,
        }
    }

    pub fn new_tank(pos: Vec2Board, current_step: BoardStep) -> Self {
        Self {
            speed: 0.2,
            health_max: 1000.,
            health: 1000.,
            pos,
            enemy_type: EnemyType::Tank,
            current_step,
            reserved_damage: 0.,
        }
    }

    pub fn distance_walked(speed: f32, dur: Duration) -> f32 {
        speed * dur.as_secs_f32()
    }

    // Return true if end is reached
    pub fn walk_until_end(&mut self, dur: Duration, board_cache: &BoardCache) -> bool {
        let mut step = &mut self.current_step;
        if step.is_end_reached() {
            if let Some(next) = board_cache.road_path.get(step.road_path_index + 1) {
                *step = next.clone();
            } else {
                return true;
            }
        } else {
            let dist = Self::distance_walked(self.speed, dur);
            step.distance_walked += dist;
            self.pos.add_in_direction(dist, step.direction);
        }
        false
    }

    pub fn is_in_range(&self, tower_pos: Vec2Board, range_radius: f32) -> bool {
        self.pos.distance(tower_pos.into()) <= range_radius
    }

    pub fn health_as_percent(&self) -> f32 {
        self.health / self.health_max
    }
}

pub(super) fn spawn_enemy_component(cmds: &mut Commands, board_visu: &BoardVisu, enemy: Enemy) {
    cmds.spawn_bundle(enemy_normal_shape(TILE_SIZE, enemy.pos.to_scaled_vec3(1.)))
        .with_children(|parent| {
            health_bar(parent, board_visu.inner_tile_size / 5.);
        })
        .insert(enemy)
        .insert(GameScreen);
}

fn enemy_normal_shape(tile_size: f32, translation: Vec3) -> ShapeBundle {
    let shape = shapes::RegularPolygon {
        sides: 5,
        feature: shapes::RegularPolygonFeature::Radius(tile_size / 8.),
        ..shapes::RegularPolygon::default()
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::MAROON),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 16.),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}
