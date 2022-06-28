use self::{
    actions::{board_editor_actions, EditorActionEvent},
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
    utils::{despawn_all_of, GameState},
};
use bevy::{prelude::*, window::WindowResized};

mod actions;
mod controls;
mod popups;
mod side_bar;
mod top_bar;

type BoardVisu = BoardVisualisation<BoardEditorScreen>;

#[derive(Component, Clone, Copy)]
struct BoardEditorScreen;

const TOP_BAR_HEIGHT_PX: f32 = 40.0;
const LEFT_BAR_WIDTH_PX: f32 = 140.0;
const EDITOR_BOARD_START: (f32, f32) = (LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX + 10.);

#[derive(Default)]
struct BoardEditorState {
    err_text: Option<String>,
}

// This plugin will contain the map editor
pub struct BoardEditorPlugin;

impl Plugin for BoardEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EditorActionEvent>()
            .add_state(SettileState::TowerGround)
            .add_system_set(SystemSet::on_enter(GameState::MapEditor).with_system(editor_setup))
            .add_system_set(
                SystemSet::on_update(GameState::MapEditor)
                    .with_system(mouse_input)
                    .with_system(on_resize)
                    .with_system(add_top_menu_bar.before(add_side_bar))
                    .with_system(add_side_bar)
                    .with_system(add_load_board_window)
                    .with_system(add_save_board_window)
                    .with_system(add_new_board_window)
                    .with_system(add_edit_board_window)
                    .with_system(board_editor_actions),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MapEditor)
                    .with_system(despawn_all_of::<BoardEditorScreen>)
                    .with_system(clean_up_editor),
            );
    }
}
fn create_visu(windows: &Windows, board: &Board) -> BoardVisu {
    BoardVisu::new(
        windows.get_primary().unwrap(),
        &board,
        EDITOR_BOARD_START.0,
        EDITOR_BOARD_START.1,
        5.,
        BoardEditorScreen,
    )
}

fn editor_setup(mut cmds: Commands, windows: Res<Windows>) {
    let board = Board::default();
    let board_cache = BoardCache::new(&board);
    let visu = create_visu(&windows, &board);
    visu.draw_board(&mut cmds, &board, &board_cache);
    cmds.insert_resource(visu);
    cmds.insert_resource(board);
    cmds.insert_resource(board_cache);
    cmds.init_resource::<BoardEditorState>();
    cmds.insert_resource(Popups::None);
}

fn on_resize(mut actions: EventWriter<EditorActionEvent>, resize_ev: EventReader<WindowResized>) {
    if !resize_ev.is_empty() {
        actions.send(EditorActionEvent::Resize);
    }
}

fn clean_up_editor(mut commands: Commands) {
    commands.remove_resource::<BoardEditorState>();
    commands.remove_resource::<Board>();
    commands.remove_resource::<BoardCache>();
    commands.remove_resource::<Popups>();
}
