use super::{set_range_circles, RangeCircleQuery};
use crate::{
    board::{visualisation::HoverCrossQuery, Board, Tile},
    game::{BoardVisu, Game},
    utils::Vec2Board,
};
use bevy::prelude::*;

type TileActionQueries<'w, 's, 'a> =
    ParamSet<'w, 's, (HoverCrossQuery<'w, 's, 'a>, RangeCircleQuery<'w, 's, 'a>)>;

pub enum TileActionsEvent {
    HoverTile(Vec2Board),
    UnhoverTile,
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn on_tile_actions(
    mut actions: EventReader<TileActionsEvent>,
    mut queries: TileActionQueries,
    board: ResMut<Board>,
    board_visu: Res<BoardVisu>,
    game: Res<Game>,
) {
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                TileActionsEvent::HoverTile(pos) => {
                    let tile = board.get_tile(&pos.as_uvec2()).unwrap();
                    on_hover_tile(&mut queries, &board_visu, &game, pos, tile)
                }
                TileActionsEvent::UnhoverTile => on_unhover_tile(&mut queries, &game),
            }
        }
    }
}

fn on_hover_tile(
    queries: &mut TileActionQueries,
    board_visu: &BoardVisu,
    game: &Game,
    pos: &Vec2Board,
    tile: &Tile,
) {
    toggle_hover_cross(&mut queries.p0(), board_visu, pos, tile);
    if !game.is_overview {
        show_range_circle(&mut queries.p1(), &pos.as_uvec2());
    }
}

fn on_unhover_tile(queries: &mut TileActionQueries, game: &Game) {
    BoardVisu::hide_hover_cross(&mut queries.p0());
    if !game.is_overview {
        set_range_circles(&mut queries.p1(), false);
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
    query.for_each_mut(|(mut visi, comp)| visi.is_visible = **comp == *pos);
}
