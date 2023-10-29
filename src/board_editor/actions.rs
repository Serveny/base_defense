use super::{popups::Popups, BoardEditor, BoardVisu};
use crate::{
    board::{
        visualisation::{BoardScreen, QueryBoardVisuTile, RoadEndMarkQuery},
        Board, BoardCache, Tile,
    },
    utils::{save_board_to_file, zoom_cam_to_board, GameState},
    CamMutQuery,
};
use bevy::prelude::*;

#[derive(Event)]
pub struct EditorSetTileEvent {
    pos: UVec2,
    tile_to: Tile,
}

impl EditorSetTileEvent {
    pub fn new(pos: UVec2, tile_to: Tile) -> Self {
        Self { pos, tile_to }
    }
}

pub(super) fn on_set_tile(
    mut evr: EventReader<EditorSetTileEvent>,
    mut editor: ResMut<BoardEditor>,
    mut board: ResMut<Board>,
    mut board_cache: ResMut<BoardCache>,
    mut q_board_visu_tile: QueryBoardVisuTile,
    mut q_road_end: RoadEndMarkQuery,
    visu: ResMut<BoardVisu>,
) {
    for ev in evr.iter() {
        set_tile(&mut board, &mut board_cache, ev.pos, ev.tile_to);
        validate_board(&mut editor, &board_cache);
        BoardVisu::change_tile(ev.pos, ev.tile_to, &mut q_board_visu_tile);
        visu.set_road_end_mark(&mut q_road_end, &board_cache)
    }
}

fn set_tile(board: &mut Board, board_cache: &mut BoardCache, pos: UVec2, tile_to: Tile) {
    if let Some(tile) = board.get_tile_mut(&pos) {
        board_cache.remove_tile_pos(&pos, tile);
        board_cache.insert_tile_pos(pos, &tile_to);
        *tile = tile_to;
        board_cache.calc_road_data(board);
    }
}

#[derive(Event)]
pub struct EditorSaveBoardEvent;

pub(super) fn on_save_board(
    mut evr: EventReader<EditorSaveBoardEvent>,
    mut popups: ResMut<Popups>,
    mut board: ResMut<Board>,
) {
    for _ in evr.iter() {
        if let Popups::Save(save_win) = &mut popups.as_mut() {
            save_win.err_text = None;
            board.name = save_win.map_file_name.clone();
            match save_board_to_file(&save_win.map_file_name, &board) {
                Ok(()) => *popups = Popups::None,
                Err(error) => save_win.err_text = Some(error.to_string()),
            }
        }
    }
}

#[derive(Event)]
pub struct EditorLoadBoardEvent(pub Board);

#[allow(clippy::too_many_arguments)]
pub(super) fn on_load_board(
    mut evr: EventReader<EditorLoadBoardEvent>,
    mut cmds: Commands,
    mut popups: ResMut<Popups>,
    mut board: ResMut<Board>,
    mut board_cache: ResMut<BoardCache>,
    mut editor: ResMut<BoardEditor>,
    mut visu: ResMut<BoardVisu>,
    mut q_cam: CamMutQuery,
    q_screen: Query<Entity, With<BoardScreen>>,
    q_win: Query<&Window>,
    assets: Res<AssetServer>,
) {
    for ev in evr.iter() {
        if let Popups::Load(_) = *popups {
            *board_cache = BoardCache::new(&ev.0);
            *board = ev.0.clone();
            *popups = Popups::None;
            validate_board(&mut editor, &board_cache);
            *visu = BoardVisu::new(0.9);
            visu.repaint(&mut cmds, &q_screen, &board, &board_cache, &assets);
            zoom_cam_to_board(&board, &mut q_cam, q_win.single());
        }
    }
}

#[derive(Event)]
pub struct EditorNewBoardEvent {
    width: u8,
    height: u8,
}

impl EditorNewBoardEvent {
    pub fn new(width: u8, lenght: u8) -> Self {
        Self {
            width,
            height: lenght,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn on_new_board(
    mut evr: EventReader<EditorNewBoardEvent>,
    mut cmds: Commands,
    mut popups: ResMut<Popups>,
    mut board: ResMut<Board>,
    mut board_cache: ResMut<BoardCache>,
    mut visu: ResMut<BoardVisu>,
    mut q_cam: CamMutQuery,
    q_screen: Query<Entity, With<BoardScreen>>,
    q_win: Query<&Window>,
    assets: Res<AssetServer>,
) {
    for ev in evr.iter() {
        if let Popups::New(_) = *popups {
            let new_board = Board::empty(ev.width, ev.height);
            *board_cache = BoardCache::new(&new_board);
            *board = new_board;
            *popups = Popups::None;
            *visu = BoardVisu::new(0.9);
            visu.repaint(&mut cmds, &q_screen, &board, &board_cache, &assets);
            zoom_cam_to_board(&board, &mut q_cam, q_win.single());
        }
    }
}

#[derive(Event)]
pub struct EditorEditBoardEvent {
    width: u8,
    height: u8,
}

impl EditorEditBoardEvent {
    pub fn new(width: u8, lenght: u8) -> Self {
        Self {
            width,
            height: lenght,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn on_edit_board(
    mut evr: EventReader<EditorEditBoardEvent>,
    mut cmds: Commands,
    mut popups: ResMut<Popups>,
    mut board: ResMut<Board>,
    mut board_cache: ResMut<BoardCache>,
    mut editor: ResMut<BoardEditor>,
    mut visu: ResMut<BoardVisu>,
    mut q_cam: CamMutQuery,
    q_screen: Query<Entity, With<BoardScreen>>,
    q_win: Query<&Window>,
    assets: Res<AssetServer>,
) {
    for ev in evr.iter() {
        if let Popups::Edit(_) = *popups {
            board.change_size(ev.width, ev.height);
            *board_cache = BoardCache::new(&board);
            *popups = Popups::None;
            validate_board(&mut editor, &board_cache);
            *visu = BoardVisu::new(0.9);
            visu.repaint(&mut cmds, &q_screen, &board, &board_cache, &assets);
            zoom_cam_to_board(&board, &mut q_cam, q_win.single());
        }
    }
}

#[derive(Event)]
pub struct EditorLeaveEvent;

pub(super) fn on_leave(
    mut evr: EventReader<EditorLeaveEvent>,
    mut set_game_state: ResMut<NextState<GameState>>,
) {
    for _ in evr.iter() {
        set_game_state.set(GameState::Menu);
    }
}

fn validate_board(editor: &mut BoardEditor, board_cache: &BoardCache) {
    editor.err_text = match board_cache.validate() {
        Ok(_) => None,
        Err(err) => Some(String::from(err)),
    };
}
