use super::{actions::EditorSetTileEvent, popups::Popups, side_bar::SettileState};
use crate::{board::Tile, utils::cursor_pos, CamQuery};
use bevy::prelude::*;

pub(super) fn mouse_input(
    set_tile_ev: EventWriter<EditorSetTileEvent>,
    set_tile_state: Res<State<SettileState>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    popups: Res<Popups>,
    wnds: Query<&Window>,
    q_cam: CamQuery,
) {
    if popups.is_open() {
        return;
    }
    if mouse_button_input.pressed(MouseButton::Left) {
        send_set_tile_event(set_tile_ev, wnds, q_cam, set_tile_state.as_tile());
    } else if mouse_button_input.pressed(MouseButton::Right) {
        send_set_tile_event(set_tile_ev, wnds, q_cam, Tile::Empty);
    }
}

fn send_set_tile_event(
    mut set_tile_ev: EventWriter<EditorSetTileEvent>,
    wnds: Query<&Window>,
    q_cam: CamQuery,
    tile: Tile,
) {
    if let Some(pos) = cursor_pos(wnds.single(), q_cam) {
        if pos.x >= 0. && pos.y >= 0. {
            set_tile_ev.send(EditorSetTileEvent::new(pos.as_uvec2(), tile));
        }
    }
}
