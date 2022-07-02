use bevy::prelude::*;
use std::time::Duration;

use crate::board::BoardCache;
use crate::utils::{IngameTime, IngameTimestamp};

use super::actions::GameActionEvent;
use super::enemies::{enemies_walk_until_wave_end, spawn_enemy_component, Enemy, EnemyType};
use super::BoardVisu;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum WaveState {
    WaveRunning,
    None,
}

#[derive(Clone)]
pub struct Wave {
    wave_no: u32,
    pub next_enemy_spawn: IngameTimestamp,
    enemies_spawned: u32,
}

impl Wave {
    pub fn new(wave_no: u32, next_enemy_spawn: IngameTimestamp) -> Self {
        Self {
            wave_no,
            enemies_spawned: 0,
            next_enemy_spawn,
        }
    }

    pub fn prepare_next_enemy_spawn(&mut self) {
        self.enemies_spawned += 1;
        self.next_enemy_spawn += Duration::from_secs_f32(2. / self.wave_no as f32);
    }

    pub fn is_wave_end(&self) -> bool {
        self.enemies_spawned >= self.wave_no * 1
    }
}

pub(super) fn wave_actions(
    mut cmds: Commands,
    mut actions: EventWriter<GameActionEvent>,
    query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
    mut wave: ResMut<Wave>,
    time: Res<Time>,
    ingame_time: Res<IngameTime>,
    board_cache: Res<BoardCache>,
    board_visu: Res<BoardVisu>,
) {
    let is_wave_end = wave.is_wave_end();
    let now = IngameTimestamp::new(ingame_time.elapsed_secs());

    // Let enemies walk
    if enemies_walk_until_wave_end(&mut cmds, query, time.delta(), &board_visu, &board_cache)
        && is_wave_end
    {
        actions.send(GameActionEvent::EndWave);
    }

    // Spawn enemy on next spawn time point
    if now >= wave.next_enemy_spawn && !is_wave_end {
        spawn_enemy_and_prepare_next(&mut cmds, &mut wave, &board_cache, &board_visu);
    }
}

fn spawn_enemy_and_prepare_next(
    cmds: &mut Commands,
    wave: &mut Wave,
    board_cache: &BoardCache,
    board_visu: &BoardVisu,
) {
    let enemy = Enemy::new(EnemyType::Normal, board_cache);
    spawn_enemy_component(cmds, board_visu, enemy);
    wave.prepare_next_enemy_spawn();
}
