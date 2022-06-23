use bevy::prelude::*;

use crate::{
    board::Tile,
    utils::{despawn_all_of, GameState},
};

use super::{
    visualisation::{HoverCross, Visualisation},
    Game, GameScreen,
};

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
    visu: Res<Visualisation>,
    game: Res<Game>,
    query_hover_cross: Query<(Entity, &mut Transform), With<HoverCross>>,
) {
    let win = windows.get_primary().unwrap();
    if let Some((pos, tile)) = get_hover_pos_and_tile(win, &visu, &game) {
        //println!("{:?}{:?}{:?}", pos, pos.as_uvec2(), tile);
        match tile {
            Tile::TowerGround(_) => visu.draw_hover_cross(&mut cmds, query_hover_cross, pos),
            Tile::BuildingGround(_) => visu.draw_hover_cross(&mut cmds, query_hover_cross, pos),
            _ => Visualisation::delete_hover_cross(&mut cmds, query_hover_cross),
        }
    } else {
        Visualisation::delete_hover_cross(&mut cmds, query_hover_cross);
    }
}

fn get_hover_pos_and_tile(win: &Window, visu: &Visualisation, game: &Game) -> Option<(Vec2, Tile)> {
    if let Some(cursor_pos) = win.cursor_position() {
        let pos = visu.cursor_px_to_board_pos(Vec2::new(
            cursor_pos.x - win.width() / 2.,
            cursor_pos.y - win.height() / 2.,
        ));
        if pos.x >= 0. && pos.y >= 0. {
            if let Some(tile) = game.action_board.try_get_tile(pos.as_uvec2()) {
                return Some((pos, tile.clone()));
            }
        }
    }
    None
}
