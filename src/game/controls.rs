use super::{BoardVisu, GameScreen};
use crate::{
    board::{visualisation::BoardHoverCross, Board, Tile},
    utils::{despawn_all_of, GameState, Vec2Board},
};
use bevy::prelude::*;

pub(super) fn keyboard_input(
    cmds: Commands,
    keys: Res<Input<KeyCode>>,
    game_state: ResMut<State<GameState>>,
    query: Query<Entity, With<GameScreen>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        end_game(cmds, game_state, query);
    }
}

fn end_game(
    cmds: Commands,
    mut game_state: ResMut<State<GameState>>,
    query: Query<Entity, With<GameScreen>>,
) {
    despawn_all_of(query, cmds);
    game_state.set(GameState::Menu).unwrap();
}

pub(super) fn mouse_input(
    mut cmds: Commands,
    windows: Res<Windows>,
    visu: Res<BoardVisu>,
    query_hover_cross: Query<(Entity, &mut Transform), With<BoardHoverCross>>,
    board: Res<Board>,
) {
    let win = windows.get_primary().unwrap();
    if let Some((pos, tile)) = get_hover_pos_and_tile(win, &visu, &board) {
        match tile {
            Tile::TowerGround(_) => visu.draw_hover_cross(&mut cmds, query_hover_cross, pos),
            Tile::BuildingGround(_) => visu.draw_hover_cross(&mut cmds, query_hover_cross, pos),
            _ => BoardVisu::delete_hover_cross(&mut cmds, query_hover_cross),
        }
    } else {
        BoardVisu::delete_hover_cross(&mut cmds, query_hover_cross);
    }
}

fn get_hover_pos_and_tile(
    win: &Window,
    visu: &BoardVisu,
    board: &Board,
) -> Option<(Vec2Board, Tile)> {
    if let Some(pos) = visu.get_hover_pos(win) {
        if pos.x >= 0. && pos.y >= 0. {
            if let Some(tile) = board.get_tile(pos.as_uvec2()) {
                return Some((pos, tile.clone()));
            }
        }
    }
    None
}
