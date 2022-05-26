use crate::board::Tile;
use bevy::prelude::*;

use super::{
    editor_tiles::{set_tile, EditorTile},
    popups::Popups,
    side_bar::{settile_state_to_tile, SettileState},
    BoardEditorState,
};

pub(super) fn editor_click_actions(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    popups: Res<Popups>,
    set_tile_state: Res<State<SettileState>>,
    state: ResMut<BoardEditorState>,
    editor_tiles: Query<(&mut Sprite, &Transform, &EditorTile), With<EditorTile>>,
) {
    if popups.is_open() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        let tile = settile_state_to_tile(set_tile_state.current().clone());
        set_tile(windows, state, editor_tiles, tile);
    } else if mouse_button_input.pressed(MouseButton::Right) {
        set_tile(windows, state, editor_tiles, Tile::Empty);
    }
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
