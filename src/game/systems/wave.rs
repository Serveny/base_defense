use crate::board::BoardCache;
use crate::game::actions::wave::WaveActionsEvent;
use crate::game::enemies::{Enemy, EnemyType};
use crate::game::Game;
use crate::utils::{IngameTime, IngameTimestamp};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum WaveState {
    Running,
    None,
}

#[derive(Resource, Clone)]
pub struct Wave {
    pub next_enemy_spawn: IngameTimestamp,
    enemies_spawned: u32,
    max_enemies: u32,
}

impl Wave {
    pub fn new(wave_no: u32, next_enemy_spawn: IngameTimestamp) -> Self {
        Self {
            enemies_spawned: 0,
            next_enemy_spawn,
            max_enemies: wave_no * 4,
        }
    }

    pub fn prepare_next_enemy_spawn(&mut self) {
        self.enemies_spawned += 1;
        // self.next_enemy_spawn += Duration::from_secs_f32(2. / self.wave_no as f32);
        self.next_enemy_spawn += Duration::from_secs_f32(0.01);
    }

    pub fn is_wave_end(&self) -> bool {
        self.enemies_spawned >= self.max_enemies
    }
}

// Tick the timer, and change state when finished
pub(in crate::game) fn wave_spawn_system(
    game: Res<Game>,
    time: Res<IngameTime>,
    mut actions: EventWriter<WaveActionsEvent>,
) {
    // Start next wave on next wave time point
    if let Some(next_wave_spawn) = game.next_wave_spawn {
        if time.elapsed_secs() >= *next_wave_spawn {
            actions.send(WaveActionsEvent::StartWave);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn wave_system(
    mut cmds: Commands,
    mut wave_acts: EventWriter<WaveActionsEvent>,
    mut wave: ResMut<Wave>,
    q_enemies: Query<&Enemy>,
    time: Res<IngameTime>,
    board_cache: Res<BoardCache>,
    wave_state: Res<State<WaveState>>,
) {
    if *wave_state.current() == WaveState::Running {
        let is_wave_end = wave.is_wave_end();
        let now = IngameTimestamp::new(time.elapsed_secs());

        // Let enemies walk
        if q_enemies.is_empty() && is_wave_end {
            wave_acts.send(WaveActionsEvent::EndWave);
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
    let enemy_type = match wave.enemies_spawned.rem_euclid(10) {
        0..=8 => EnemyType::Normal,
        _ => EnemyType::Tank,
    };
    if let Some(enemy) = Enemy::new(enemy_type, q_enemies, board_cache) {
        enemy.spawn(cmds);
        wave.prepare_next_enemy_spawn();
    }
}
