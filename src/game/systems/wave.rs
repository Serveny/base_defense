use crate::board::BoardCache;
use crate::game::actions::resources::ResourcesEvent;
use crate::game::actions::wave::WaveActionsEvent;
use crate::game::enemies::{spawn_enemy_component, Enemy, EnemyType};
use crate::game::{BoardVisu, Game};
use crate::utils::{IngameTime, IngameTimestamp};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Clone, Eq, PartialEq, Debug, Hash, PartialOrd, Ord)]
pub enum WaveState {
    Running,
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
        self.enemies_spawned >= self.wave_no * 4
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
    res_acts: EventWriter<ResourcesEvent>,
    query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
    time: Res<Time>,
    ingame_time: Res<IngameTime>,
    board_cache: Res<BoardCache>,
    board_visu: Res<BoardVisu>,
    wave_state: Res<State<WaveState>>,
) {
    if *wave_state.current() == WaveState::Running {
        let is_wave_end = wave.is_wave_end();
        let now = IngameTimestamp::new(ingame_time.elapsed_secs());

        // Let enemies walk
        if enemies_walk_until_wave_end(&mut cmds, res_acts, query, time.delta(), &board_cache)
            && is_wave_end
        {
            wave_acts.send(WaveActionsEvent::EndWave);
        }

        // Spawn enemy on next spawn time point
        if !is_wave_end && now >= wave.next_enemy_spawn {
            spawn_enemy_and_prepare_next(&mut cmds, &mut wave, &board_cache, &board_visu);
        }
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

pub(super) fn enemies_walk_until_wave_end(
    cmds: &mut Commands,
    mut res_actions: EventWriter<ResourcesEvent>,
    mut query: Query<(Entity, &mut Enemy, &mut Transform), With<Enemy>>,
    dur: Duration,
    board_cache: &BoardCache,
) -> bool {
    query.for_each_mut(|(entity, mut enemy, mut transform)| {
        match enemy.walk_until_end(dur, board_cache) {
            true => enemy_reached_base(cmds, &mut res_actions, &enemy, entity),
            false => transform.translation = enemy.pos.to_scaled_vec3(1.),
        }
    });
    query.is_empty()
}

fn enemy_reached_base(
    cmds: &mut Commands,
    res_actions: &mut EventWriter<ResourcesEvent>,
    enemy: &Enemy,
    entity: Entity,
) {
    let damage = (-enemy.health * 20.).round();
    res_actions.send(ResourcesEvent::Energy(damage, enemy.pos));
    res_actions.send(ResourcesEvent::Materials(damage, enemy.pos));
    cmds.entity(entity).despawn_recursive();
}
