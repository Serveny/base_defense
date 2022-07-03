use super::actions::GameActionEvent;
use crate::{
    board::{Board, Tile},
    utils::{cursor_pos, Vec2Board},
    CamQuery,
};
use bevy::prelude::*;

pub(super) fn keyboard_input(keys: Res<Input<KeyCode>>, mut actions: EventWriter<GameActionEvent>) {
    if keys.just_released(KeyCode::Escape) {
        actions.send(GameActionEvent::BackToMainMenu);
    }
    if keys.just_pressed(KeyCode::LShift) {
        actions.send(GameActionEvent::ActivateOverview);
    }
    if keys.just_released(KeyCode::LShift) {
        actions.send(GameActionEvent::DeactivateOverview);
    }
}

pub(super) fn mouse_input(
    mut actions: EventWriter<GameActionEvent>,
    wnds: Res<Windows>,
    q_cam: CamQuery,
    board: Res<Board>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if let Some((pos, tile)) = get_hover_pos_and_tile(wnds, q_cam, board) {
        actions.send(GameActionEvent::HoverTile(pos, tile));

        if mouse_button_input.pressed(MouseButton::Left) {
            actions.send(GameActionEvent::TileLeftClick(pos.as_uvec2()));
        }
    } else {
        actions.send(GameActionEvent::UnhoverTile);
    }
}

fn get_hover_pos_and_tile(
    wnds: Res<Windows>,
    q_cam: CamQuery,
    board: Res<Board>,
) -> Option<(Vec2Board, Tile)> {
    if let Some(pos) = cursor_pos(wnds, q_cam) {
        if pos.x >= 0. && pos.y >= 0. {
            if let Some(tile) = board.get_tile(&pos.as_uvec2()) {
                return Some((pos, tile.clone()));
            }
        }
    }
    None
}
