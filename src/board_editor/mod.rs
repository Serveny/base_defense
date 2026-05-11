use self::{
    actions::{
        EditorEditBoardMessage, EditorLeaveMessage, EditorLoadBoardMessage, EditorNewBoardMessage,
        EditorSaveBoardMessage, EditorSetTileMessage,
    },
    controls::mouse_input,
    popups::{
        add_edit_board_window, add_load_board_window, add_new_board_window, add_save_board_window,
        Popups,
    },
    side_bar::{add_side_bar, SettileState},
    top_bar::add_top_menu_bar,
};
use crate::{
    board::{visualisation::BoardVisualisation, Board, BoardCache},
    utils::{despawn_all_of, zoom_cam_to_board_with_viewport_padding, GameState},
    CamMutQuery,
};
use bevy::{prelude::*, window::WindowResized};
use bevy_egui::EguiPrimaryContextPass;

mod actions;
mod controls;
mod popups;
mod side_bar;
mod top_bar;

type BoardVisu = BoardVisualisation<BoardEditorScreen>;

#[derive(Component, Clone, Copy, Default)]
struct BoardEditorScreen;

const TOP_BAR_HEIGHT_PX: f32 = 40.0;
const LEFT_BAR_WIDTH_PX: f32 = 140.0;
// const EDITOR_BOARD_START: (f32, f32) = (LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX + 10.);

#[derive(Resource, Default)]
struct BoardEditor {
    err_text: Option<String>,
}

// This plugin will contain the map editor
pub struct BoardEditorPlugin;

impl Plugin for BoardEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<EditorSetTileMessage>()
            .add_message::<EditorSaveBoardMessage>()
            .add_message::<EditorLoadBoardMessage>()
            .add_message::<EditorNewBoardMessage>()
            .add_message::<EditorEditBoardMessage>()
            .add_message::<EditorLeaveMessage>()
            .init_state::<SettileState>()
            .add_systems(OnEnter(GameState::MapEditor), editor_setup)
            .add_systems(
                Update,
                (
                    on_resize,
                    mouse_input,
                    actions::on_set_tile,
                    actions::on_save_board,
                    actions::on_load_board,
                    actions::on_new_board,
                    actions::on_edit_board,
                    actions::on_leave,
                )
                    .run_if(in_state(GameState::MapEditor)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                (
                    add_top_menu_bar.before(add_side_bar),
                    add_side_bar,
                    add_load_board_window,
                    add_save_board_window,
                    add_new_board_window,
                    add_edit_board_window,
                )
                    .run_if(in_state(GameState::MapEditor)),
            )
            .add_systems(
                OnExit(GameState::MapEditor),
                (despawn_all_of::<BoardEditorScreen>, clean_up_editor),
            );
    }
}

fn editor_setup(
    mut cmds: Commands,
    mut q_cam: CamMutQuery,
    q_win: Query<&Window>,
    assets: Res<AssetServer>,
) {
    let board = Board::default();
    let board_cache = BoardCache::new(&board);

    zoom_cam_to_editor_board(&board, &mut q_cam, q_win);
    let visu = BoardVisu::new(0.9);
    visu.draw_board(&mut cmds, &board, &board_cache, &assets);
    cmds.insert_resource(visu);
    cmds.insert_resource(board);
    cmds.insert_resource(board_cache);
    cmds.init_resource::<BoardEditor>();
    cmds.insert_resource(Popups::None);
}

fn on_resize(
    mut ev: MessageReader<WindowResized>,
    mut q_cam: CamMutQuery,
    q_win: Query<&Window>,
    board: Res<Board>,
) {
    for _ in ev.read() {
        zoom_cam_to_editor_board(&board, &mut q_cam, q_win);
    }
}

pub(super) fn zoom_cam_to_editor_board(
    board: &Board,
    q_cam: &mut CamMutQuery,
    q_win: Query<&Window>,
) {
    zoom_cam_to_board_with_viewport_padding(
        board,
        q_cam,
        q_win,
        Vec2::new(LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX),
    );
}

fn clean_up_editor(mut commands: Commands) {
    commands.remove_resource::<BoardEditor>();
    commands.remove_resource::<Board>();
    commands.remove_resource::<BoardCache>();
    commands.remove_resource::<Popups>();
}
