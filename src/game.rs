use std::time::Duration;

use self::{
    actions::{build_menu::BuildMenuCloseEvent, GameActions},
    build_menus::{draw_build_menu, BuildMenu, BuildMenuScreen},
    controls::{keyboard_input, mouse_input},
    statistics::{EnemyKillCount, LaserShotsFired, RocketsFired},
    systems::{
        game_over::GameOverTimer,
        wave::{Wave, WaveState},
        GameSystems,
    },
};
use crate::{
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
mod statistics;
mod systems;

type BoardVisu = BoardVisualisation<GameScreen>;
type BaseLevel = u8;

#[derive(States, Component, Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
enum IngameState {
    #[default]
    None,
    Running,
    Pause,
    GameOver,
}

#[derive(Resource, Default, Clone)]
struct HoveredTile(Option<(Vec2Board, Tile)>);

pub const GAME_OVER_COUNTDOWN_TIME: Duration = Duration::from_secs(60);
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<IngameState>()
            .add_plugins((GameSystems, GameActions))
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(
                Update,
                (keyboard_input, on_resize).run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (tick_ingame_timer, mouse_input).run_if(in_state(IngameState::Running)),
            )
            .add_systems(
                OnExit(GameState::Game),
                (
                    clean_up_game,
                    despawn_all_of::<GameScreen>,
                    despawn_all_of::<BuildMenuScreen>,
                ),
            );
    }
}

#[allow(dead_code)]
#[derive(Resource, Clone)]
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
    mut evr: EventReader<WindowResized>,
    mut q_cam: CamMutQuery,
    wins: Query<&Window>,
    board: Res<Board>,
) {
    for _ in evr.iter() {
        zoom_cam_to_board(&board, &mut q_cam, wins.single());
    }
}

#[allow(clippy::too_many_arguments)]
fn game_setup(
    mut cmds: Commands,
    mut set_ingame_state: ResMut<NextState<IngameState>>,
    mut q_cam: CamMutQuery,
    bm_close_ev: EventWriter<BuildMenuCloseEvent>,
    wins: Query<&Window>,
    board: Res<Board>,
    board_cache: Res<BoardCache>,
    game: Res<Game>,
    assets: Res<AssetServer>,
) {
    zoom_cam_to_board(&board, &mut q_cam, wins.single());
    let visu = BoardVisu::new(1.);
    visu.draw_board(&mut cmds, &board, &board_cache, &assets);
    draw_build_menu(&mut cmds, bm_close_ev, game.base_lvl);

    cmds.insert_resource(visu);
    cmds.init_resource::<IngameTime>();
    cmds.init_resource::<BuildMenu>();
    cmds.init_resource::<Collisions>();
    cmds.init_resource::<GameOverTimer>();
    cmds.init_resource::<HoveredTile>();
    cmds.init_resource::<EnemyKillCount>();
    cmds.init_resource::<LaserShotsFired>();
    cmds.init_resource::<RocketsFired>();

    set_ingame_state.set(IngameState::Running);
}

fn tick_ingame_timer(mut timer: ResMut<IngameTime>, time: Res<Time>, game: Res<Game>) {
    timer.tick(Duration::from_secs_f32(time.delta_seconds() * game.speed));
}

fn clean_up_game(
    mut cmds: Commands,
    mut set_wave_state: ResMut<NextState<WaveState>>,
    mut set_ingame_state: ResMut<NextState<IngameState>>,
) {
    set_wave_state.set(WaveState::None);
    set_ingame_state.set(IngameState::None);
    cmds.remove_resource::<Game>();
    cmds.remove_resource::<Board>();
    cmds.remove_resource::<BoardCache>();
    cmds.remove_resource::<BoardVisu>();
    cmds.remove_resource::<Wave>();
    cmds.remove_resource::<IngameTime>();
    cmds.remove_resource::<BuildMenu>();
    cmds.remove_resource::<GameOverTimer>();
    cmds.remove_resource::<EnemyKillCount>();
    cmds.remove_resource::<LaserShotsFired>();
    cmds.remove_resource::<RocketsFired>();
}
