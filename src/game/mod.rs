use self::{
    actions::{tower_menu::TowerMenuActionsEvent, GameActions},
    controls::{keyboard_input, mouse_input},
    systems::{wave::Wave, GameSystems},
    tower_build_menu::{draw_tower_build_menu, TowerMenu, TowerMenuScreen},
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
mod tower_build_menu;

type BoardVisu = BoardVisualisation<GameScreen>;
type BaseLevel = u8;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GameSystems)
            .add_plugin(GameActions)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(tick_ingame_timer)
                    .with_system(keyboard_input)
                    .with_system(mouse_input)
                    .with_system(on_resize),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(clean_up_game)
                    .with_system(despawn_all_of::<GameScreen>)
                    .with_system(despawn_all_of::<TowerMenuScreen>),
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
    base_lvl: BaseLevel,
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
            base_lvl: 1,
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
        zoom_cam_to_board(&board, cam, &wins);
    }
}

#[allow(clippy::too_many_arguments)]
fn game_setup(
    mut cmds: Commands,
    tm_ev: EventWriter<TowerMenuActionsEvent>,
    cam_query: CamMutQuery,
    windows: Res<Windows>,
    board: Res<Board>,
    board_cache: Res<BoardCache>,
    game: Res<Game>,
) {
    zoom_cam_to_board(&board, cam_query, &windows);
    let visu = BoardVisu::new(1.);
    visu.draw_board(&mut cmds, &board, &board_cache);
    draw_tower_build_menu(&mut cmds, tm_ev, game.base_lvl);
    cmds.insert_resource(visu);
    cmds.init_resource::<IngameTime>();
    cmds.init_resource::<TowerMenu>();
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
    cmds.remove_resource::<TowerMenu>();
}
