use super::{actions::EditorActionEvent, popups::Popups, side_bar::SettileState, Visu};
use crate::board::Tile;
use bevy::prelude::*;

pub(super) fn mouse_input(
    actions: EventWriter<EditorActionEvent>,
    set_tile_state: Res<State<SettileState>>,
    mouse_button_input: Res<Input<MouseButton>>,
    popups: Res<Popups>,
    visu: Res<Visu>,
    windows: Res<Windows>,
) {
    if popups.is_open() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        send_set_tile_event(actions, windows, visu, set_tile_state.current().as_tile());
    } else if mouse_button_input.pressed(MouseButton::Right) {
        send_set_tile_event(actions, windows, visu, Tile::Empty);
    }
}

fn send_set_tile_event(
    mut actions: EventWriter<EditorActionEvent>,
    windows: Res<Windows>,
    visu: Res<Visu>,
    tile: Tile,
) {
    let win = windows.get_primary().unwrap();
    if let Some(pos) = visu.get_hover_pos(win) {
        if pos.x >= 0. && pos.y >= 0. {
            actions.send(EditorActionEvent::SetTile(pos.as_uvec2(), tile));
        }
    }
}
