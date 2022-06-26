use crate::{
    board::{Board, BoardCache},
    utils::GameState,
};

use super::{GameScreen, Visu};
use bevy::prelude::*;

pub(super) enum GameActionEvent {
    Resize,
}

#[allow(dead_code)]
struct GameActionParams<'w, 's, 'gs, 'visu, 'b, 'bc, 'win> {
    cmds: Commands<'w, 's>,
    game_state: &'gs mut State<GameState>,
    visu: &'visu mut Visu,
    board: &'b mut Board,
    board_cache: &'bc mut BoardCache,
    windows: &'win Windows,
}

pub(super) fn game_actions(
    cmds: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut visu: ResMut<Visu>,
    mut board: ResMut<Board>,
    mut board_cache: ResMut<BoardCache>,
    mut queries: ParamSet<(Query<Entity, With<GameScreen>>,)>,
    mut game_actions: EventReader<GameActionEvent>,
    windows: Res<Windows>,
) {
    if !game_actions.is_empty() {
        let mut ga_params = GameActionParams {
            cmds: cmds,
            game_state: &mut game_state,
            visu: &mut visu,
            board: &mut board,
            board_cache: &mut board_cache,
            windows: &windows,
        };
        for event in game_actions.iter() {
            match event {
                GameActionEvent::Resize => repaint(&mut ga_params, queries.p0()),
            }
        }
    }
}

fn repaint(ga_params: &mut GameActionParams, query: Query<Entity, With<GameScreen>>) {
    *ga_params.visu = create_visu(ga_params.windows, ga_params.board);
    ga_params.visu.repaint(
        &mut ga_params.cmds,
        query,
        ga_params.board,
        ga_params.board_cache,
    );
}

fn create_visu(windows: &Windows, board: &Board) -> Visu {
    Visu::new(
        windows.get_primary().unwrap(),
        &board,
        0.,
        0.,
        0.,
        GameScreen,
    )
}
