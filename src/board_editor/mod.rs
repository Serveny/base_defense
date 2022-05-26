use crate::{
    assets::StandardAssets,
    board::Board,
    utils::{despawn_all_of, GameState},
};
use bevy::{prelude::*, window::WindowResized};

use self::{
    actions::editor_click_actions,
    editor_tiles::{resize_tiles, spawn_tiles, EditorTile, TileResizeParams},
    markers::{resize_markers, spawn_markers, EditorRoadEndPoint, EditorRoadStartPoint},
    popups::{
        add_edit_board_window, add_load_board_window, add_new_board_window, add_save_board_window,
        Popups,
    },
    side_bar::{add_side_bar, SettileState},
    top_bar::add_top_menu_bar,
};

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

#[derive(Default)]
struct BoardEditorState {
    current_map: Board,
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
fn editor_setup(mut commands: Commands, windows: Res<Windows>, assets: Res<StandardAssets>) {
    let mut editor_state = BoardEditorState::default();
    spawn_tiles(&mut commands, &windows, &mut editor_state.current_map);
    spawn_markers(&mut commands, &assets);

    commands.insert_resource(editor_state);
    commands.insert_resource(Popups::None);
}

fn on_resize(
    events: EventReader<WindowResized>,
    editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
    // road_start: Query<&mut Transform, With<EditorRoadStartPoint>>,
    // road_end: Query<&mut Transform, With<EditorRoadEndPoint>>,
    windows: Res<Windows>,
    state: Res<BoardEditorState>,
) {
    if !events.is_empty() {
        let rs_params = TileResizeParams::new(&windows, &state.current_map);
        resize_tiles(&rs_params, editor_tiles);
        //resize_markers(&rs_params, road_start, road_end, &state);
    }
}

fn repaint(
    commands: &mut Commands,
    editor_tiles: &mut Query<Entity, With<EditorTile>>,
    windows: &Windows,
    state: &mut BoardEditorState,
) {
    for entity in editor_tiles.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    spawn_tiles(commands, windows, &mut state.current_map);
}

fn clean_up_editor(mut commands: Commands) {
    commands.remove_resource::<BoardEditorState>();
    commands.remove_resource::<Popups>();
}
