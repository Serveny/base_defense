use super::{BoardVisu, Game, GameScreen};
use crate::{
    board::{step::BoardStep, BoardCache},
    utils::{enemy_normal_shape, Vec2Board},
};
use bevy::prelude::*;
use std::time::{Duration, Instant};

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

pub(super) fn spawn_enemies(
    cmds: &mut Commands,
    game: &mut Game,
    visu: &BoardVisu,
    last_update: Instant,
    board_cache: &BoardCache,
) {
    if let Some(next) = &mut game.wave.next_enemy_spawn {
        if last_update >= *next {
            spawn_enemy(cmds, visu, Enemy::new(EnemyType::Normal, &board_cache));
            game.wave.enemies_spawned += 1;
            if game.wave.enemies_spawned < game.wave.wave_no * 4 {
                *next += Duration::from_secs_f32(2. / game.wave.wave_no as f32);
            } else {
                game.wave.next_enemy_spawn = None;
            }
        }
    }
}

fn spawn_enemy(cmds: &mut Commands, visu: &BoardVisu, enemy: Enemy) {
    cmds.spawn_bundle(enemy_normal_shape(
        visu.tile_size,
        visu.pos_to_px(enemy.pos, 1.),
    ))
    .insert(enemy)
    .insert(GameScreen);
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

pub(super) fn resize_enemies(
    cmds: &mut Commands,
    visu: &BoardVisu,
    query: Query<(&Enemy, Entity), With<Enemy>>,
) {
    let mut enemies = Vec::<Enemy>::new();
    query.for_each(|(enemy, entity)| {
        enemies.push(enemy.clone());
        cmds.entity(entity).despawn_recursive();
    });
    for enemy in enemies {
        spawn_enemy(cmds, &visu, enemy);
    }
}
