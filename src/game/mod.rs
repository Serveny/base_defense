use std::time::Duration;

use self::{
    actions::{game_actions, GameActionEvent},
    controls::{keyboard_input, mouse_input},
    tower_system::tower_system,
    wave::{wave_actions, WaveState},
};
use crate::{
    board::{visualisation::BoardVisualisation, Board, BoardCache},
    utils::{despawn_all_of, Difficulty, Energy, Materials},
    GameState,
};
use bevy::{prelude::*, utils::Instant, window::WindowResized};

mod actions;
mod controls;
mod enemies;
mod tower_system;
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
