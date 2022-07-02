use self::{
    actions::{game_actions, GameActionEvent},
    controls::{keyboard_input, mouse_input},
    tower_system::tower_system,
    wave::{wave_actions, WaveState},
};
use crate::{
    board::{visualisation::BoardVisualisation, Board, BoardCache},
    utils::{despawn_all_of, Difficulty, Energy, IngameTime, IngameTimestamp, Materials},
    zoom_cam_to_board, CamQuery, GameState,
};
use bevy::{prelude::*, render::camera::Camera2d, window::WindowResized};

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
                    .with_system(tick_ingame_timer)
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

fn on_resize(
    mut resize_ev: EventReader<WindowResized>,
    windows: Res<Windows>,
    query: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    if !resize_ev.is_empty() {
        let win = windows.get_primary().unwrap();
        for ev in resize_ev.iter() {
            println!("{:?}\n", win);
            println!("{:?}\n", ev);
            println!("\n");
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct Game {
    difficulty: Difficulty,
    energy: Energy,
    materials: Materials,
    wave_no: u32,
    next_wave_spawn: Option<IngameTimestamp>,
    is_overview: bool,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            energy: 100,
            materials: 100,
            wave_no: 0,
            next_wave_spawn: Some(IngameTimestamp::new(1.)),
            is_overview: false,
        }
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component, Clone, Copy, Default)]
struct GameScreen;

fn game_setup(
    mut cmds: Commands,
    cam_query: CamQuery,
    windows: Res<Windows>,
    board: Res<Board>,
    board_cache: Res<BoardCache>,
) {
    zoom_cam_to_board(&board, cam_query, &windows);
    let visu = BoardVisu::new(1.);
    visu.draw_board(&mut cmds, &board, &board_cache);
    cmds.insert_resource(visu);
    cmds.init_resource::<IngameTime>();
}

// Tick the timer, and change state when finished
fn wave_spawn_system(
    game: Res<Game>,
    time: Res<IngameTime>,
    mut actions: EventWriter<GameActionEvent>,
) {
    // Start next wave on next wave time point
    if let Some(next_wave_spawn) = game.next_wave_spawn {
        if time.elapsed_secs() >= *next_wave_spawn {
            actions.send(GameActionEvent::StartWave);
        }
    }
}

fn tick_ingame_timer(mut timer: ResMut<IngameTime>, time: Res<Time>) {
    timer.tick(time.delta());
}
