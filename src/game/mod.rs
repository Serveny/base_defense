use std::time::Duration;

use self::{
    actions::{game_actions, GameActionEvent},
    controls::{keyboard_input, mouse_input},
    enemies::Enemy,
    wave::{wave_actions, WaveState},
};
use crate::{
    board::{visualisation::BoardVisualisation, Board, BoardCache},
    utils::{
        despawn_all_of,
        towers::{pos_to_quat, Tower, TowerCannon, TowerValues},
        Difficulty, Energy, Materials, Vec2Board,
    },
    GameState,
};
use bevy::{prelude::*, utils::Instant, window::WindowResized};

mod actions;
mod controls;
mod enemies;
mod wave;

type BoardVisu = BoardVisualisation<GameScreen>;
type EnemiesQuery<'w, 's, 'a> = Query<'w, 's, (Entity, &'a Enemy), With<Enemy>>;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent>()
            .add_state(WaveState::None)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(keyboard_input)
                    .with_system(mouse_input)
                    .with_system(on_resize)
                    .with_system(wave_spawn_system)
                    .with_system(tower_system)
                    .with_system(game_actions),
            )
            .add_system_set(
                SystemSet::on_update(WaveState::WaveRunning)
                    .with_system(wave_actions.before(game_actions)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_all_of::<GameScreen>),
            );
    }
}

fn on_resize(mut actions: EventWriter<GameActionEvent>, resize_ev: EventReader<WindowResized>) {
    if !resize_ev.is_empty() {
        actions.send(GameActionEvent::Resize);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct Game {
    difficulty: Difficulty,
    energy: Energy,
    materials: Materials,
    wave_no: u32,
    next_wave_spawn: Option<Instant>,
    is_overview: bool,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            energy: 100,
            materials: 100,
            wave_no: 0,
            next_wave_spawn: Some(Instant::now() + Duration::from_secs(1)),
            is_overview: false,
        }
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component, Clone, Copy)]
struct GameScreen;

fn game_setup(
    mut cmds: Commands,
    windows: Res<Windows>,
    board: Res<Board>,
    board_cache: Res<BoardCache>,
) {
    let win = windows.get_primary().unwrap();
    let visu = BoardVisu::new(win, &board, 0., 0., 0., GameScreen);
    visu.draw_board(&mut cmds, &board, &board_cache);
    cmds.insert_resource(visu);
}

// Tick the timer, and change state when finished
fn wave_spawn_system(game: Res<Game>, time: Res<Time>, mut actions: EventWriter<GameActionEvent>) {
    if let Some(last_update) = time.last_update() {
        // Start next wave on next wave time point
        if let Some(next_wave_spawn) = game.next_wave_spawn {
            if last_update >= next_wave_spawn {
                actions.send(GameActionEvent::StartWave);
            }
        }
    }
}

fn tower_system(
    mut cannon_transforms: Query<&mut Transform, With<TowerCannon>>,
    mut towers: Query<(&mut Tower, &Children), With<Tower>>,
    enemies: EnemiesQuery,
) {
    for (mut tower, children) in towers.iter_mut() {
        rotate_tower_cannon_to_enemies(&mut cannon_transforms, children, &mut tower, &enemies);
    }
}

fn rotate_tower_cannon_to_enemies(
    cannon_transforms: &mut Query<&mut Transform, With<TowerCannon>>,
    tower_children: &Children,
    tower: &mut Tower,
    enemies: &EnemiesQuery,
) {
    let tower_vals = tower.values_mut();
    if let Some(locked_entity) = tower_vals.target_lock {
        if let Some(locked_enemy) = find_locked_enemy(locked_entity, &enemies) {
            rotate_tower_cannon_to_pos(
                cannon_transforms,
                tower_vals.pos,
                locked_enemy.pos,
                tower_children,
            );
        } else {
            tower_vals.target_lock = None;
        }
    } else {
        tower_vals.target_lock = find_first_enemy_entity_in_range(&tower_vals, enemies);
    }
}

fn find_first_enemy_entity_in_range<'a>(
    tower_vals: &TowerValues,
    enemies: &'a EnemiesQuery,
) -> Option<Entity> {
    for (entity, enemy) in enemies.iter() {
        if is_enemy_in_tower_range(enemy.pos, tower_vals.pos, tower_vals.range_radius) {
            return Some(entity);
        }
    }
    None
}

fn find_locked_enemy<'a>(locked_entity: Entity, enemies: &'a EnemiesQuery) -> Option<&'a Enemy> {
    for (entity, enemy) in enemies.iter() {
        if entity == locked_entity {
            return Some(enemy);
        }
    }
    None
}

fn is_enemy_in_tower_range(enemy_pos: Vec2Board, tower_pos: Vec2Board, radius: f32) -> bool {
    enemy_pos.distance(tower_pos.into()) <= radius
}

fn rotate_tower_cannon_to_pos(
    cannon_transforms: &mut Query<&mut Transform, With<TowerCannon>>,
    tower_pos: Vec2Board,
    enemy_pos: Vec2Board,
    tower_children: &Children,
) {
    for child in tower_children.iter() {
        if let Ok(mut transform) = cannon_transforms.get_mut(*child) {
            transform.rotation = pos_to_quat(tower_pos, enemy_pos);
        }
    }
}
