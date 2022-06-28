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
        towers::{Tower, TowerCannon, TowerValues},
        Difficulty, Energy, Materials, Vec2Board,
    },
    GameState,
};
use bevy::{prelude::*, utils::Instant, window::WindowResized};
use euclid::Angle;

mod actions;
mod controls;
mod enemies;
mod wave;

type BoardVisu = BoardVisualisation<GameScreen>;

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
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            energy: 100,
            materials: 100,
            wave_no: 0,
            next_wave_spawn: Some(Instant::now() + Duration::from_secs(1)),
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
    towers: Query<(&Tower, &Children), With<Tower>>,
    enemies: Query<&Enemy, With<Enemy>>,
) {
    for (tower, children) in towers.iter() {
        let tower_vals = tower.values();
        for enemy in enemies.iter() {
            if is_enemy_in_tower_range(enemy.pos, tower_vals.pos, tower_vals.range_radius) {
                rotate_tower_cannon_to_pos(&tower_vals, &enemy, children, &mut cannon_transforms);
            }
        }
    }
}

fn is_enemy_in_tower_range(enemy_pos: Vec2Board, tower_pos: Vec2Board, radius: f32) -> bool {
    enemy_pos.distance(tower_pos) <= radius
}

fn rotate_tower_cannon_to_pos(
    tower_vals: &TowerValues,
    enemy: &Enemy,
    tower_children: &Children,
    cannon_transforms: &mut Query<&mut Transform, With<TowerCannon>>,
) {
    let x = tower_vals.pos.angle_between(enemy.pos.into());

    // println!("{:?}", Angle::radians(x).to_degrees());
    // Visualisation
    for child in tower_children.iter() {
        if let Ok(mut trans) = cannon_transforms.get_mut(*child) {
            trans.rotate(Quat::from_rotation_z(std::f32::consts::PI / 180.))
        }
    }
}
