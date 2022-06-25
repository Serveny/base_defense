use super::MenuState;
use crate::{
    board::{Board, BoardCache},
    game::Game,
    utils::GameState,
};
use bevy::prelude::*;

pub(super) enum MenuActionEvent {
    StartNewGame(Game, Board, BoardCache),
    LeaveMenu(GameState),
}

struct MenuActionParams<'w, 's, 'ms, 'gs> {
    cmds: Commands<'w, 's>,
    menu_state: &'ms mut State<MenuState>,
    game_state: &'gs mut State<GameState>,
}

pub(super) fn menu_actions(
    cmds: Commands,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<GameState>>,
    mut menu_actions: EventReader<MenuActionEvent>,
) {
    if !menu_actions.is_empty() {
        let mut ma_params = MenuActionParams {
            cmds: cmds,
            menu_state: &mut menu_state,
            game_state: &mut game_state,
        };
        for event in menu_actions.iter() {
            match event {
                MenuActionEvent::StartNewGame(game, board, board_cache) => {
                    start_new_game(
                        &mut ma_params,
                        game.clone(),
                        board.clone(),
                        board_cache.clone(),
                    );
                }
                MenuActionEvent::LeaveMenu(to) => leave_menu(&mut ma_params, to.clone()),
            }
        }
    }
}

fn start_new_game(
    ma_params: &mut MenuActionParams,
    game: Game,
    board: Board,
    board_cache: BoardCache,
) {
    ma_params.cmds.insert_resource(game);
    ma_params.cmds.insert_resource(board);
    ma_params.cmds.insert_resource(board_cache);
    leave_menu(ma_params, GameState::Game);
}

fn leave_menu(ma_params: &mut MenuActionParams, to: GameState) {
    ma_params
        .menu_state
        .set(MenuState::Main)
        .unwrap_or_default();
    ma_params.game_state.set(to).unwrap();
}
