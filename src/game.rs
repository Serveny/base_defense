use std::time::Duration;

use self::{
    actions::{build_menu::BuildMenuActionsEvent, GameActions},
    build_menus::{draw_build_menu, BuildMenu, BuildMenuScreen},
    controls::{keyboard_input, mouse_input},
    systems::{
        game_over::GameOverTimer,
        wave::{Wave, WaveState},
        GameSystems,
    },
};
use crate::{
    assets::StandardAssets,
    board::{visualisation::BoardVisualisation, Board, BoardCache, Tile},
    utils::{
        collision::Collisions, despawn_all_of, zoom_cam_to_board, Difficulty, Energy, IngameTime,
        IngameTimestamp, Materials, Vec2Board,
    },
    CamMutQuery, GameState,
};
use bevy::{prelude::*, window::WindowResized};

mod actions;
mod build_menus;
mod controls;
mod enemies;
mod systems;

type BoardVisu = BoardVisualisation<GameScreen>;
type BaseLevel = u8;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum IngameState {
    Running,
    Pause,
    GameOver,
    None,
}

#[derive(Default, Clone)]
struct HoveredTile(Option<(Vec2Board, Tile)>);

pub const GAME_OVER_COUNTDOWN_TIME: Duration = Duration::from_secs(60);
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(IngameState::None)
            .add_plugin(GameSystems)
            .add_plugin(GameActions)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(keyboard_input)
                    .with_system(on_resize),
            )
            .add_system_set(
                SystemSet::on_update(IngameState::Running)
                    .with_system(tick_ingame_timer)
                    .with_system(mouse_input),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(clean_up_game)
                    .with_system(despawn_all_of::<GameScreen>)
                    .with_system(despawn_all_of::<BuildMenuScreen>),
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
    speed: f32,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            difficulty,
            energy: 1000.,
            materials: 1000.,
            wave_no: 0,
            next_wave_spawn: Some(IngameTimestamp::new(1.)),
            is_overview: false,
            base_lvl: 1,
            speed: 1.,
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
    mut ingame_state: ResMut<State<IngameState>>,
    tm_ev: EventWriter<BuildMenuActionsEvent>,
    cam_query: CamMutQuery,
    windows: Res<Windows>,
    board: Res<Board>,
    board_cache: Res<BoardCache>,
    game: Res<Game>,
    assets: Res<StandardAssets>,
) {
    zoom_cam_to_board(&board, cam_query, &windows);
    let visu = BoardVisu::new(1.);
    visu.draw_board(&mut cmds, &board, &board_cache, &assets);
    draw_build_menu(&mut cmds, tm_ev, game.base_lvl);

    cmds.insert_resource(visu);
    cmds.init_resource::<IngameTime>();
    cmds.init_resource::<BuildMenu>();
    cmds.init_resource::<Collisions>();
    cmds.init_resource::<GameOverTimer>();
    cmds.init_resource::<HoveredTile>();

    ingame_state.set(IngameState::Running).unwrap();
}

fn tick_ingame_timer(mut timer: ResMut<IngameTime>, time: Res<Time>, game: Res<Game>) {
    timer.tick(Duration::from_secs_f32(time.delta_seconds() * game.speed));
}

fn clean_up_game(
    mut cmds: Commands,
    mut wave_state: ResMut<State<WaveState>>,
    mut ingame_state: ResMut<State<IngameState>>,
) {
    wave_state.set(WaveState::None).unwrap_or_default();
    ingame_state.set(IngameState::None).unwrap();
    cmds.remove_resource::<Game>();
    cmds.remove_resource::<Board>();
    cmds.remove_resource::<BoardCache>();
    cmds.remove_resource::<BoardVisu>();
    cmds.remove_resource::<Wave>();
    cmds.remove_resource::<IngameTime>();
    cmds.remove_resource::<BuildMenu>();
    cmds.remove_resource::<GameOverTimer>();
}
