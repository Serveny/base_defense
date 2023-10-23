use super::{new_game_menu::NewGameMenu, MenuState};
use crate::{
    board::{Board, BoardCache},
    game::Game,
    utils::GameState,
};
use bevy::prelude::*;

#[allow(clippy::large_enum_variant)]
#[derive(Event)]
pub(super) enum MenuActionEvent {
    EnterNewGameMenu,
    StartNewGame(Game, Board, BoardCache),
    LeaveMenu(GameState),
}

struct MenuActionParams<'w, 's, 'a> {
    cmds: Commands<'w, 's>,
    menu_state: &'a State<MenuState>,
    set_menu_state: &'a mut NextState<MenuState>,
    set_game_state: &'a mut NextState<GameState>,
}

pub(super) fn menu_actions(
    cmds: Commands,
    menu_state: Res<State<MenuState>>,
    mut set_menu_state: ResMut<NextState<MenuState>>,
    mut set_game_state: ResMut<NextState<GameState>>,
    mut menu_actions: EventReader<MenuActionEvent>,
) {
    if !menu_actions.is_empty() {
        let mut ma_params = MenuActionParams {
            cmds,
            menu_state: &menu_state,
            set_menu_state: &mut set_menu_state,
            set_game_state: &mut set_game_state,
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
                MenuActionEvent::LeaveMenu(to) => leave_menu(&mut ma_params, *to),
                MenuActionEvent::EnterNewGameMenu => enter_new_game_menu(&mut ma_params),
            }
        }
    }
}

fn enter_new_game_menu(ma_params: &mut MenuActionParams) {
    ma_params.cmds.init_resource::<NewGameMenu>();
    match **ma_params.menu_state {
        MenuState::NewGame => {
            ma_params.cmds.remove_resource::<NewGameMenu>();
            ma_params.set_menu_state.set(MenuState::Main);
        }
        _ => ma_params.set_menu_state.set(MenuState::NewGame),
    };
}

fn start_new_game(
    ma_params: &mut MenuActionParams,
    game: Game,
    board: Board,
    board_cache: BoardCache,
) {
    ma_params.cmds.remove_resource::<NewGameMenu>();
    ma_params.cmds.insert_resource(game);
    ma_params.cmds.insert_resource(board);
    ma_params.cmds.insert_resource(board_cache);
    leave_menu(ma_params, GameState::Game);
}

fn leave_menu(ma_params: &mut MenuActionParams, to: GameState) {
    ma_params.set_menu_state.set(MenuState::Main);
    ma_params.set_game_state.set(to);
}
