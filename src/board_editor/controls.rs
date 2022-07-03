use super::{actions::EditorActionEvent, popups::Popups, side_bar::SettileState};
use crate::{board::Tile, utils::cursor_pos, CamQuery};
use bevy::prelude::*;

pub(super) fn mouse_input(
    actions: EventWriter<EditorActionEvent>,
    set_tile_state: Res<State<SettileState>>,
    mouse_button_input: Res<Input<MouseButton>>,
    popups: Res<Popups>,
    wnds: Res<Windows>,
    q_cam: CamQuery,
) {
    if popups.is_open() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        send_set_tile_event(actions, wnds, q_cam, set_tile_state.current().as_tile());
    } else if mouse_button_input.pressed(MouseButton::Right) {
        send_set_tile_event(actions, wnds, q_cam, Tile::Empty);
    }
}

fn send_set_tile_event(
    mut actions: EventWriter<EditorActionEvent>,
    wnds: Res<Windows>,
    q_cam: CamQuery,
    tile: Tile,
) {
    if let Some(pos) = cursor_pos(wnds, q_cam) {
        if pos.x >= 0. && pos.y >= 0. {
            actions.send(EditorActionEvent::SetTile(pos.as_uvec2(), tile));
        }
    }
}
