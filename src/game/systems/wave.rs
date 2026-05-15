use crate::balance::{
    TANK_SPAWN_EVERY_NTH_ENEMY, WAVE_BASE_ENEMY_COUNT, WAVE_ENEMIES_PER_WAVE,
    WAVE_ENEMY_SPAWN_INTERVAL_DECREASE_PER_WAVE_SECS, WAVE_MIN_ENEMY_SPAWN_INTERVAL_SECS,
    WAVE_START_ENEMY_SPAWN_INTERVAL_SECS,
};
use crate::board::BoardCache;
use crate::game::actions::wave::WaveActionsMessage;
use crate::game::enemies::{Enemy, EnemyType};
use crate::game::Game;
use crate::utils::{IngameTime, IngameTimestamp};
use bevy::prelude::*;
use std::time::Duration;

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, PartialOrd, Ord, Default)]
pub enum WaveState {
    #[default]
    None,
    Running,
}

#[derive(Resource, Clone)]
pub struct Wave {
    pub next_enemy_spawn: IngameTimestamp,
    enemies_spawned: u32,
    max_enemies: u32,
    enemy_spawn_interval: Duration,
}

impl Wave {
    pub fn new(wave_no: u32, next_enemy_spawn: IngameTimestamp) -> Self {
        let spawn_interval_secs = (WAVE_START_ENEMY_SPAWN_INTERVAL_SECS
            - (wave_no.saturating_sub(1) as f32
                * WAVE_ENEMY_SPAWN_INTERVAL_DECREASE_PER_WAVE_SECS))
            .max(WAVE_MIN_ENEMY_SPAWN_INTERVAL_SECS);
        Self {
            enemies_spawned: 0,
            next_enemy_spawn,
            max_enemies: WAVE_BASE_ENEMY_COUNT + (wave_no * WAVE_ENEMIES_PER_WAVE),
            enemy_spawn_interval: Duration::from_secs_f32(spawn_interval_secs),
        }
    }

    pub fn prepare_next_enemy_spawn(&mut self) {
        self.enemies_spawned += 1;
        self.next_enemy_spawn += self.enemy_spawn_interval;
    }

    pub fn is_wave_end(&self) -> bool {
        self.enemies_spawned >= self.max_enemies
    }
}

// Tick the timer, and change state when finished
pub(in crate::game) fn wave_spawn_system(
    game: Res<Game>,
    time: Res<IngameTime>,
    mut actions: MessageWriter<WaveActionsMessage>,
) {
    // Start next wave on next wave time point
    if let Some(next_wave_spawn) = game.next_wave_spawn {
        if time.elapsed_secs() >= *next_wave_spawn {
            actions.write(WaveActionsMessage::StartWave);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn wave_system(
    mut cmds: Commands,
    mut wave_acts: MessageWriter<WaveActionsMessage>,
    mut wave: ResMut<Wave>,
    q_enemies: Query<&Enemy>,
    time: Res<IngameTime>,
    board_cache: Res<BoardCache>,
    wave_state: Res<State<WaveState>>,
) {
    if *wave_state == WaveState::Running {
        let is_wave_end = wave.is_wave_end();
        let now = IngameTimestamp::new(time.elapsed_secs());

        // Let enemies walk
        if q_enemies.is_empty() && is_wave_end {
            wave_acts.write(WaveActionsMessage::EndWave);
        }

        // Spawn enemy on next spawn time point
        if !is_wave_end && now >= wave.next_enemy_spawn {
            spawn_enemy_and_prepare_next(&mut cmds, &mut wave, &q_enemies, &board_cache);
        }
    }
}

fn spawn_enemy_and_prepare_next(
    cmds: &mut Commands,
    wave: &mut Wave,
    q_enemies: &Query<&Enemy>,
    board_cache: &BoardCache,
) {
    let enemy_type = match (wave.enemies_spawned + 1).rem_euclid(TANK_SPAWN_EVERY_NTH_ENEMY) {
        0 => EnemyType::Tank,
        _ => EnemyType::Normal,
    };
    if let Some(enemy) = Enemy::new(enemy_type, q_enemies, board_cache) {
        enemy.spawn(cmds);
        wave.prepare_next_enemy_spawn();
    }
}
