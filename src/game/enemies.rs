use super::{BoardVisu, GameScreen};
use crate::{
    board::{step::BoardStep, BoardCache},
    utils::{health_bar::health_bar, Vec2Board},
};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use std::time::Duration;

#[derive(Clone)]
pub enum EnemyType {
    Normal,
    Speeder,
    Tank,
}

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct Enemy {
    speed: f32,
    health: u32,
    pub pos: Vec2Board,
    enemy_type: EnemyType,
    current_step: BoardStep,
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
            health: 100,
            pos,
            enemy_type: EnemyType::Normal,
            current_step,
        }
    }

    pub fn new_speeder(pos: Vec2Board, current_step: BoardStep) -> Self {
        Self {
            speed: 2.,
            health: 10,
            pos,
            enemy_type: EnemyType::Speeder,
            current_step,
        }
    }

    pub fn new_tank(pos: Vec2Board, current_step: BoardStep) -> Self {
        Self {
            speed: 0.2,
            health: 1000,
            pos,
            enemy_type: EnemyType::Tank,
            current_step,
        }
    }

    pub fn distance_walked(speed: f32, dur: Duration) -> f32 {
        speed * dur.as_secs_f64() as f32
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
}

pub(super) fn enemies_walk_until_wave_end(
    cmds: &mut Commands,
    mut query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
    dur: Duration,
    visu: &BoardVisu,
    board_cache: &BoardCache,
) -> bool {
    query.for_each_mut(|(entity, mut enemy, mut transform)| {
        if enemy.walk_until_end(dur, board_cache) {
            cmds.entity(entity).despawn_recursive();
        } else {
            transform.translation = visu.pos_to_px(enemy.pos, 1.);
        }
    });
    query.is_empty()
}

pub(super) fn spawn_enemy_component(cmds: &mut Commands, board_visu: &BoardVisu, enemy: Enemy) {
    cmds.spawn_bundle(enemy_normal_shape(
        board_visu.tile_size,
        board_visu.pos_to_px(enemy.pos, 1.),
    ))
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
