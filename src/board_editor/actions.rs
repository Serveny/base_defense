use super::{
    editor_tiles::{set_tile, EditorTile, TileResizeParams},
    markers::{set_mark, BoardEditorRoadEndMark, BoardEditorRoadStartMark},
    popups::Popups,
    side_bar::{settile_state_to_tile, SettileState},
    BoardEditorState,
};
use crate::board::Tile;
use bevy::prelude::*;

pub(super) fn editor_click_actions(
    commands: Commands,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    popups: Res<Popups>,
    set_tile_state: Res<State<SettileState>>,
    state: ResMut<BoardEditorState>,
    queries: ParamSet<(
        Query<(&mut Sprite, &Transform, &EditorTile), With<EditorTile>>,
        Query<(Entity, &mut Transform), With<BoardEditorRoadStartMark>>,
        Query<(Entity, &mut Transform), With<BoardEditorRoadEndMark>>,
    )>,
) {
    if popups.is_open() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        let tile = settile_state_to_tile(set_tile_state.current().clone());
        on_tile_click(commands, windows, queries, state, tile);
    } else if mouse_button_input.pressed(MouseButton::Right) {
        on_tile_click(commands, windows, queries, state, Tile::Empty);
    }
}

fn on_tile_click(
    mut commands: Commands,
    windows: Res<Windows>,
    mut queries: ParamSet<(
        Query<(&mut Sprite, &Transform, &EditorTile), With<EditorTile>>,
        Query<(Entity, &mut Transform), With<BoardEditorRoadStartMark>>,
        Query<(Entity, &mut Transform), With<BoardEditorRoadEndMark>>,
    )>,
    mut state: ResMut<BoardEditorState>,
    tile_to: Tile,
) {
    let rs_params = TileResizeParams::new(&windows, state.current_map.board());
    set_tile(windows, &mut state, queries.p0(), tile_to);
    set_mark(
        &mut commands,
        queries.p1().get_single_mut().ok(),
        state.current_map.road_start_pos().clone(),
        &rs_params,
        true,
    );
    set_mark(
        &mut commands,
        queries.p2().get_single_mut().ok(),
        state.current_map.road_end_pos().clone(),
        &rs_params,
        false,
    );
}

pub(super) fn is_hover(cursor_pos: Vec2, sprite: &Sprite, transform: &Transform) -> bool {
    if let Some(size) = sprite.custom_size {
        cursor_pos.x >= transform.translation.x
            && cursor_pos.x <= transform.translation.x + size.x
            && cursor_pos.y >= transform.translation.y - size.y
            && cursor_pos.y <= transform.translation.y
    } else {
        false
    }
}

pub(super) fn cursor_pos_to_transform_pos(cursor_pos: Vec2, window: &Window) -> Vec2 {
    Vec2::new(
        cursor_pos.x - (window.width() / 2.),
        cursor_pos.y - (window.height() / 2.),
    )
}
