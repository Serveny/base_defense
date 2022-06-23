use self::{
    actions::editor_click_actions,
    editor_tiles::spawn_tiles,
    markers::spawn_end_marker,
    popups::{
        add_edit_board_window, add_load_board_window, add_new_board_window, add_save_board_window,
        Popups,
    },
    side_bar::{add_side_bar, SettileState},
    top_bar::add_top_menu_bar,
};
use crate::{
    board::ActionBoard,
    utils::{despawn_all_of, GameState, TileResizeParams},
};
use bevy::{prelude::*, window::WindowResized};

mod actions;
mod editor_tiles;
mod markers;
mod popups;
mod side_bar;
mod top_bar;

#[derive(Component)]
struct BoardEditorScreen;

const TOP_BAR_HEIGHT_PX: f32 = 40.0;
const LEFT_BAR_WIDTH_PX: f32 = 140.0;
const EDITOR_BOARD_START: (f32, f32) = (LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX);

#[derive(Default)]
struct BoardEditorState {
    current_map: ActionBoard,
    err_text: Option<String>,
}

// This plugin will contain the map editor
pub struct BoardEditorPlugin;

impl Plugin for BoardEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(SettileState::TowerGround)
            .add_system_set(SystemSet::on_enter(GameState::MapEditor).with_system(editor_setup))
            .add_system_set(
                SystemSet::on_update(GameState::MapEditor)
                    .with_system(add_top_menu_bar.before(add_side_bar))
                    .with_system(add_side_bar)
                    .with_system(on_resize)
                    .with_system(editor_click_actions)
                    .with_system(add_load_board_window)
                    .with_system(add_save_board_window)
                    .with_system(add_new_board_window)
                    .with_system(add_edit_board_window),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MapEditor)
                    .with_system(despawn_all_of::<BoardEditorScreen>)
                    .with_system(clean_up_editor),
            );
    }
}

fn editor_setup(mut commands: Commands, windows: Res<Windows>) {
    let state = BoardEditorState::default();
    let window = windows.get_primary().unwrap();
    let rs_params = TileResizeParams::from_start_to_win_end(
        window,
        state.current_map.board(),
        Vec2::from(EDITOR_BOARD_START),
    );
    spawn_tiles(&mut commands, &rs_params, state.current_map.board());

    commands.insert_resource(state);
    commands.insert_resource(Popups::None);
}

fn on_resize(
    mut commands: Commands,
    events: EventReader<WindowResized>,
    query: Query<Entity, With<BoardEditorScreen>>,
    windows: Res<Windows>,
    mut state: ResMut<BoardEditorState>,
) {
    if !events.is_empty() {
        repaint(&mut commands, query, &windows, &mut state);
    }
}

fn repaint(
    commands: &mut Commands,
    mut query: Query<Entity, With<BoardEditorScreen>>,
    windows: &Windows,
    state: &mut BoardEditorState,
) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    let window = windows.get_primary().unwrap();
    let rs_params = TileResizeParams::from_start_to_win_end(
        window,
        state.current_map.board(),
        Vec2::from(EDITOR_BOARD_START),
    );
    spawn_tiles(commands, &rs_params, &mut state.current_map.board());

    if let Some(end_mark) = state.current_map.road_end_pos() {
        spawn_end_marker(commands, &rs_params, end_mark.clone());
    }
}

fn clean_up_editor(mut commands: Commands) {
    commands.remove_resource::<BoardEditorState>();
    commands.remove_resource::<Popups>();
}
