use super::{popups::Popups, BoardEditor, BoardVisu};
use crate::{
    assets::StandardAssets,
    board::{
        visualisation::{BoardScreen, BoardScreenQuery, BoardVisualTile, RoadEndMarkQuery},
        Board, BoardCache, Tile,
    },
    utils::{save_board_to_file, zoom_cam_to_board, GameState},
    CamMutQuery,
};
use bevy::prelude::*;

pub(super) enum EditorActionEvent {
    SetTile(UVec2, Tile),
    Load(Board),
    Save,
    New((u8, u8)),
    Edit((u8, u8)),
    Leave,
}

type EditorActionQueries<'w, 's, 'a> = ParamSet<
    'w,
    's,
    (
        Query<'w, 's, (&'a mut Sprite, &'a Transform, &'a BoardVisualTile), With<BoardVisualTile>>,
        RoadEndMarkQuery<'w, 's, 'a>,
        Query<'w, 's, Entity, With<BoardScreen>>,
        CamMutQuery<'w, 's, 'a>,
    ),
>;

struct EditorActionParams<'w, 's, 'gs, 'es, 'visu, 'b, 'bc, 'win, 'pu, 'ass> {
    cmds: Commands<'w, 's>,
    game_state: &'gs mut State<GameState>,
    editor: &'es mut BoardEditor,
    visu: &'visu mut BoardVisu,
    board: &'b mut Board,
    board_cache: &'bc mut BoardCache,
    windows: &'win Windows,
    popups: &'pu mut Popups,
    assets: &'ass StandardAssets,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn board_editor_actions(
    cmds: Commands,
    mut game_state: ResMut<State<GameState>>,
    mut editor: ResMut<BoardEditor>,
    mut visu: ResMut<BoardVisu>,
    mut board: ResMut<Board>,
    mut board_cache: ResMut<BoardCache>,
    mut queries: EditorActionQueries,
    mut popups: ResMut<Popups>,
    mut editor_actions: EventReader<EditorActionEvent>,
    windows: Res<Windows>,
    assets: Res<StandardAssets>,
) {
    if !editor_actions.is_empty() {
        let mut ea_params = EditorActionParams {
            cmds,
            game_state: &mut game_state,
            editor: &mut editor,
            visu: &mut visu,
            board: &mut board,
            board_cache: &mut board_cache,
            windows: &windows,
            popups: &mut popups,
            assets: &assets,
        };
        for event in editor_actions.iter() {
            match event {
                EditorActionEvent::SetTile(pos, tile_to) => {
                    set_tile_and_update_mark(&mut ea_params, &mut queries, pos, tile_to)
                }
                EditorActionEvent::Save => save_board(&mut ea_params),
                EditorActionEvent::Load(board) => {
                    load_board(&mut ea_params, &mut queries, board.clone())
                }
                EditorActionEvent::New(size) => new_board(&mut ea_params, &mut queries, *size),
                EditorActionEvent::Edit(size) => edit_board(&mut ea_params, &mut queries, *size),
                EditorActionEvent::Leave => leave(&mut ea_params),
            }
        }
    }
}

fn set_tile_and_update_mark(
    ea_params: &mut EditorActionParams,
    queries: &mut EditorActionQueries,
    pos: &UVec2,
    tile_to: &Tile,
) {
    set_tile(
        ea_params.board,
        ea_params.board_cache,
        *pos,
        tile_to.clone(),
    );
    validate_board(ea_params);
    BoardVisu::change_tile(pos, tile_to, queries.p0());
    ea_params
        .visu
        .set_road_end_mark(queries.p1(), ea_params.board_cache);
}

fn set_tile(board: &mut Board, board_cache: &mut BoardCache, pos: UVec2, tile_to: Tile) {
    if let Some(tile) = board.get_tile_mut(&pos) {
        board_cache.remove_tile_pos(&pos, tile);
        board_cache.insert_tile_pos(pos, &tile_to);
        *tile = tile_to;
        board_cache.calc_road_data(board);
    }
}

fn repaint(ea_params: &mut EditorActionParams, query: BoardScreenQuery, assets: &StandardAssets) {
    *ea_params.visu = BoardVisu::new(0.9);
    ea_params.visu.repaint(
        &mut ea_params.cmds,
        query,
        ea_params.board,
        ea_params.board_cache,
        assets,
    );
}

fn save_board(ea_params: &mut EditorActionParams) {
    if let Popups::Save(save_win) = ea_params.popups {
        save_win.err_text = None;
        ea_params.board.name = save_win.map_file_name.clone();
        match save_board_to_file(&save_win.map_file_name, ea_params.board) {
            Ok(()) => *ea_params.popups = Popups::None,
            Err(error) => save_win.err_text = Some(error.to_string()),
        }
    }
}

fn load_board(
    ea_params: &mut EditorActionParams,
    queries: &mut EditorActionQueries,
    new_board: Board,
) {
    if let Popups::Load(_) = ea_params.popups {
        *ea_params.board_cache = BoardCache::new(&new_board);
        *ea_params.board = new_board;
        *ea_params.popups = Popups::None;
        validate_board(ea_params);
        repaint(ea_params, queries.p2(), ea_params.assets);
        zoom_cam_to_board(ea_params.board, queries.p3(), ea_params.windows);
    }
}

fn new_board(
    ea_params: &mut EditorActionParams,
    queries: &mut EditorActionQueries,
    size: (u8, u8),
) {
    if let Popups::New(_) = ea_params.popups {
        let new_board = Board::empty(size.0, size.1);
        *ea_params.board_cache = BoardCache::new(&new_board);
        *ea_params.board = new_board;
        *ea_params.popups = Popups::None;
        repaint(ea_params, queries.p2(), ea_params.assets);
        zoom_cam_to_board(ea_params.board, queries.p3(), ea_params.windows);
    }
}

fn edit_board(
    ea_params: &mut EditorActionParams,
    queries: &mut EditorActionQueries,
    size: (u8, u8),
) {
    if let Popups::Edit(_) = ea_params.popups {
        ea_params.board.change_size(size.0, size.1);
        *ea_params.board_cache = BoardCache::new(ea_params.board);
        *ea_params.popups = Popups::None;
        validate_board(ea_params);
        repaint(ea_params, queries.p2(), ea_params.assets);
        zoom_cam_to_board(ea_params.board, queries.p3(), ea_params.windows);
    }
}

fn leave(ea_params: &mut EditorActionParams) {
    ea_params.game_state.set(GameState::Menu).unwrap();
}

fn validate_board(ea_params: &mut EditorActionParams) {
    ea_params.editor.err_text = match ea_params.board_cache.validate() {
        Ok(_) => None,
        Err(err) => Some(String::from(err)),
    };
}
