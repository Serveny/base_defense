#![allow(unused)]
use std::time::{Duration, Instant};

use bevy::prelude::*;
use indexmap::IndexSet;

use crate::{
    board::ActionBoard,
    utils::{enemy_normal_shape, Vec2Board},
};

use super::{visualisation::Visualisation, Game, GameScreen, Wave};

pub enum EnemyType {
    Normal,
    Speeder,
    Tank,
}

#[derive(Component)]
pub struct Enemy {
    speed: f32,
    health: u32,
    pub pos: Vec2Board,
    enemy_type: EnemyType,
    vec_rel_to_next_road_tile: Vec2Board,
    next_road_tile: Option<(usize, Vec2Board)>,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, road_tile_posis: &IndexSet<UVec2>) -> Self {
        let first_middle = Vec2Board::from_uvec2_middle(road_tile_posis.get_index(0).unwrap());
        let second_middle = Vec2Board::from_uvec2_middle(road_tile_posis.get_index(1).unwrap());
        match enemy_type {
            EnemyType::Normal => Self::new_normal(first_middle, second_middle),
            EnemyType::Speeder => Self::new_speeder(first_middle, second_middle),
            EnemyType::Tank => Self::new_tank(first_middle, second_middle),
        }
    }

    pub fn new_normal(pos: Vec2Board, next_road_tile: Vec2Board) -> Self {
        Self {
            speed: 1.,
            health: 100,
            pos,
            enemy_type: EnemyType::Normal,
            vec_rel_to_next_road_tile: next_road_tile - pos,
            next_road_tile: Some((1, next_road_tile)),
        }
    }

    pub fn new_speeder(pos: Vec2Board, next_road_tile: Vec2Board) -> Self {
        Self {
            speed: 2.,
            health: 10,
            pos,
            enemy_type: EnemyType::Speeder,
            vec_rel_to_next_road_tile: next_road_tile - pos,
            next_road_tile: Some((1, next_road_tile)),
        }
    }

    pub fn new_tank(pos: Vec2Board, next_road_tile: Vec2Board) -> Self {
        Self {
            speed: 0.2,
            health: 1000,
            pos,
            enemy_type: EnemyType::Tank,
            vec_rel_to_next_road_tile: next_road_tile - pos,
            next_road_tile: Some((1, next_road_tile)),
        }
    }

    pub fn distance_walked(&self, dur: Duration) -> f64 {
        self.speed as f64 * dur.as_secs_f64()
    }

    // Return true if end is reached
    pub fn walk_until_end(&mut self, dur: Duration, road_tiles: &IndexSet<UVec2>) -> bool {
        if self.is_step_end_reached() {
            if let Some(new_pos) = self.next_road_tile {
                self.pos = new_pos.1;
                self.next_road_tile = Self::get_next_road_tile(new_pos.0 + 1, road_tiles);
                if let Some(next) = self.next_road_tile {
                    self.vec_rel_to_next_road_tile = next.1 - self.pos;
                } else {
                    return true;
                }
            }
        } else {
            let dist = self.distance_walked(dur) as f32;
            let dist_vec = self.vec_rel_to_next_road_tile * Vec2Board::new(dist, dist);
            // println!("{:?}", x);
            self.pos = self.pos + dist_vec;
        }
        false
    }

    fn is_step_end_reached(&self) -> bool {
        if let Some(next) = self.next_road_tile {
            return (self.vec_rel_to_next_road_tile.x < 0. && self.pos.x <= next.1.x)
                || (self.vec_rel_to_next_road_tile.x > 0. && self.pos.x >= next.1.x)
                || (self.vec_rel_to_next_road_tile.y < 0. && self.pos.y <= next.1.y)
                || (self.vec_rel_to_next_road_tile.y > 0. && self.pos.y >= next.1.y);
        }
        false
    }
    fn get_next_road_tile(
        index: usize,
        road_tiles: &IndexSet<UVec2>,
    ) -> Option<(usize, Vec2Board)> {
        if let Some(new_next_tile) = road_tiles.get_index(index) {
            return Some((index, Vec2Board::from_uvec2_middle(new_next_tile)));
        }
        None
    }
}

pub(super) fn spawn_enemies(
    cmds: &mut Commands,
    game: &mut Game,
    visu: &Visualisation,
    last_update: Instant,
) {
    if let Some(next) = &mut game.wave.next_enemy_spawn {
        if last_update >= *next {
            spawn_enemy(cmds, &game.action_board, visu, EnemyType::Normal);
            game.wave.enemies_spawned += 1;
            if game.wave.enemies_spawned < game.wave.wave_no * 4 {
                *next += Duration::from_secs_f32(2. / game.wave.wave_no as f32);
            } else {
                game.wave.next_enemy_spawn = None;
            }
        }
    }
}

fn spawn_enemy(
    cmds: &mut Commands,
    action_board: &ActionBoard,
    visu: &Visualisation,
    enemy_type: EnemyType,
) {
    cmds.spawn_bundle(enemy_normal_shape(
        visu.tile_size,
        visu.pos_to_px(action_board.road_start_pos().unwrap().into(), 1.) + visu.half_tile_vec3,
    ))
    .insert(Enemy::new(enemy_type, action_board.road_tile_posis()))
    .insert(GameScreen);
}
pub(super) fn enemies_walk_until_wave_end(
    cmds: &mut Commands,
    dur: Duration,
    mut query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
    visu: &Visualisation,
    road_tile_posis: &IndexSet<UVec2>,
) -> bool {
    query.for_each_mut(|(mut entity, mut enemy, mut transform)| {
        if enemy.walk_until_end(dur, road_tile_posis) {
            cmds.entity(entity).despawn_recursive();
        } else {
            transform.translation = visu.pos_to_px(enemy.pos, 1.);
        }
    });
    query.is_empty()
}
