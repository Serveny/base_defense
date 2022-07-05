use self::{
    actions::{
        game_actions,
        tile::{on_tile_actions, TileActionsEvent},
        tower::{on_tower_actions, TowerActionsEvent},
        wave::{on_wave_actions, WaveActionsEvent},
        GameActionEvent,
    },
    controls::{keyboard_input, mouse_input},
    systems::{
        health_bar::health_bar_system,
        shot::shot_system,
        tower::tower_system,
        wave::{wave_spawn_system, wave_system, Wave, WaveState},
    },
};
use crate::{
    board::{visualisation::BoardVisualisation, Board, BoardCache},
    utils::{
        despawn_all_of, zoom_cam_to_board, Difficulty, Energy, IngameTime, IngameTimestamp,
        Materials,
    },
    CamMutQuery, GameState,
};
use bevy::{prelude::*, window::WindowResized};

mod actions;
mod controls;
mod enemies;
mod systems;

type BoardVisu = BoardVisualisation<GameScreen>;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent>()
            .add_event::<TileActionsEvent>()
            .add_event::<TowerActionsEvent>()
            .add_event::<WaveActionsEvent>()
            .add_state(WaveState::None)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(tick_ingame_timer)
                    .with_system(keyboard_input)
                    .with_system(mouse_input)
                    .with_system(on_resize)
                    .with_system(wave_spawn_system)
                    .with_system(shot_system)
                    .with_system(tower_system)
                    .with_system(health_bar_system)
                    .with_system(game_actions)
                    .with_system(on_tower_actions)
                    .with_system(on_wave_actions)
                    .with_system(on_tile_actions),
            )
            .add_system_set(
                SystemSet::on_update(WaveState::Running)
                    .with_system(wave_system.before(game_actions).before(on_wave_actions)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(clean_up_game)
                    .with_system(despawn_all_of::<GameScreen>),
            );
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

fn on_resize(
    ev: EventReader<WindowResized>,
    wins: Res<Windows>,
    board: Res<Board>,
    cam: CamMutQuery,
) {
    if !ev.is_empty() {
        zoom_cam_to_board(&board, cam, &wins, Vec2::default());
    }
}

fn game_setup(
    mut cmds: Commands,
    cam_query: CamMutQuery,
    windows: Res<Windows>,
    board: Res<Board>,
    board_cache: Res<BoardCache>,
) {
    zoom_cam_to_board(&board, cam_query, &windows, Vec2::default());
    let visu = BoardVisu::new(1.);
    visu.draw_board(&mut cmds, &board, &board_cache);
    cmds.insert_resource(visu);
    cmds.init_resource::<IngameTime>();
}

fn tick_ingame_timer(mut timer: ResMut<IngameTime>, time: Res<Time>) {
    timer.tick(time.delta());
}

fn clean_up_game(mut cmds: Commands) {
    cmds.remove_resource::<Game>();
    cmds.remove_resource::<Board>();
    cmds.remove_resource::<BoardCache>();
    cmds.remove_resource::<BoardVisu>();
    cmds.remove_resource::<Wave>();
    cmds.remove_resource::<IngameTime>();
}
