use super::{set_range_circles, RangeCircleQuery};
use crate::{
    board::{visualisation::HoverCrossQuery, Board, Tile},
    game::{BoardVisu, Game},
    utils::{visible, Vec2Board},
};
use bevy::prelude::*;

#[derive(Event)]
pub enum TileActionsEvent {
    HoverTile(Vec2Board),
    UnhoverTile,
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn on_tile_actions(
    mut evr: EventReader<TileActionsEvent>,
    mut q_hover_cross: HoverCrossQuery,
    mut q_range_circle: RangeCircleQuery,
    board: ResMut<Board>,
    board_visu: Res<BoardVisu>,
    game: Res<Game>,
) {
    for action in evr.read() {
        match action {
            TileActionsEvent::HoverTile(pos) => {
                let tile = board.get_tile(&pos.as_uvec2()).unwrap();
                on_hover_tile(
                    &mut q_hover_cross,
                    &mut q_range_circle,
                    &board_visu,
                    &game,
                    pos,
                    tile,
                )
            }
            TileActionsEvent::UnhoverTile => {
                on_unhover_tile(&mut q_hover_cross, &mut q_range_circle, &game)
            }
        }
    }
}

fn on_hover_tile(
    q_hover_cross: &mut HoverCrossQuery,
    q_range_circle: &mut RangeCircleQuery,
    board_visu: &BoardVisu,
    game: &Game,
    pos: &Vec2Board,
    tile: &Tile,
) {
    toggle_hover_cross(q_hover_cross, board_visu, pos, tile);
    if !game.is_overview {
        show_range_circle(q_range_circle, &pos.as_uvec2());
    }
}

fn on_unhover_tile(
    q_hover_cross: &mut HoverCrossQuery,
    q_range_circle: &mut RangeCircleQuery,
    game: &Game,
) {
    BoardVisu::hide_hover_cross(q_hover_cross);
    if !game.is_overview {
        set_range_circles(q_range_circle, false);
    }
}

fn toggle_hover_cross(
    query: &mut HoverCrossQuery,
    board_visu: &BoardVisu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    match tile {
        Tile::TowerGround => board_visu.show_hover_cross(query, pos),
        Tile::BuildingGround => board_visu.show_hover_cross(query, pos),
        _ => BoardVisu::hide_hover_cross(query),
    }
}

fn show_range_circle(query: &mut RangeCircleQuery, pos: &UVec2) {
    query
        .iter_mut()
        .for_each(|(mut visi, comp)| *visi = visible(**comp == *pos));
}
