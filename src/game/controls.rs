use super::{actions::GameActionEvent, BoardVisu};
use crate::{
    board::{Board, Tile},
    utils::Vec2Board,
};
use bevy::prelude::*;

pub(super) fn keyboard_input(keys: Res<Input<KeyCode>>, mut actions: EventWriter<GameActionEvent>) {
    if keys.just_released(KeyCode::Escape) {
        actions.send(GameActionEvent::BackToMainMenu);
    }
}

pub(super) fn mouse_input(
    mut actions: EventWriter<GameActionEvent>,
    windows: Res<Windows>,
    visu: Res<BoardVisu>,
    board: Res<Board>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    let win = windows.get_primary().unwrap();
    if let Some((pos, tile)) = get_hover_pos_and_tile(win, &visu, &board) {
        actions.send(GameActionEvent::HoverTile(pos, tile));

        if mouse_button_input.pressed(MouseButton::Left) {
            actions.send(GameActionEvent::TileLeftClick(pos.as_uvec2()));
        }
    } else {
        actions.send(GameActionEvent::UnhoverTile);
    }
}

fn get_hover_pos_and_tile(
    win: &Window,
    visu: &BoardVisu,
    board: &Board,
) -> Option<(Vec2Board, Tile)> {
    if let Some(pos) = visu.get_hover_pos(win) {
        if pos.x >= 0. && pos.y >= 0. {
            if let Some(tile) = board.get_tile(&pos.as_uvec2()) {
                return Some((pos, tile.clone()));
            }
        }
    }
    None
}
