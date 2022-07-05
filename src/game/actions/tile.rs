use bevy::prelude::*;

use crate::{
    board::{visualisation::HoverCrossQuery, Board, Tile},
    game::{BoardVisu, Game},
    utils::{
        towers::{draw_tower, Tower},
        IngameTime, IngameTimestamp, Vec2Board,
    },
};

use super::{set_range_circles, RangeCircleQuery};

type TileActionQueries<'w, 's, 'a> =
    ParamSet<'w, 's, (HoverCrossQuery<'w, 's, 'a>, RangeCircleQuery<'w, 's, 'a>)>;

pub enum TileActionsEvent {
    HoverTile(Vec2Board, Tile),
    UnhoverTile,
    TileLeftClick(UVec2),
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn on_tile_actions(
    mut cmds: Commands,
    mut actions: EventReader<TileActionsEvent>,
    mut board: ResMut<Board>,
    mut queries: TileActionQueries,
    board_visu: Res<BoardVisu>,
    game: Res<Game>,
    time: Res<IngameTime>,
) {
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                TileActionsEvent::HoverTile(pos, tile) => {
                    on_hover_tile(&mut queries, &board_visu, &game, pos, tile)
                }
                TileActionsEvent::UnhoverTile => on_unhover_tile(&mut queries, &game),
                TileActionsEvent::TileLeftClick(pos) => {
                    handle_click(&mut cmds, &mut board, &board_visu, time.now(), pos)
                }
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
    draw_hover_cross(&mut queries.p0(), board_visu, pos, tile);
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

fn draw_hover_cross(
    query: &mut HoverCrossQuery,
    board_visu: &BoardVisu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    match tile {
        Tile::TowerGround(_) => board_visu.show_hover_cross(query, pos),
        Tile::BuildingGround(_) => board_visu.show_hover_cross(query, pos),
        _ => BoardVisu::hide_hover_cross(query),
    }
}

fn show_range_circle(query: &mut RangeCircleQuery, pos: &UVec2) {
    query.for_each_mut(|(mut visi, comp)| visi.is_visible = **comp == *pos);
}

fn handle_click(
    cmds: &mut Commands,
    board: &mut Board,
    board_visu: &BoardVisu,
    now: IngameTimestamp,
    pos: &UVec2,
) {
    if let Some(tile) = board.get_tile_mut(pos) {
        on_tile_click(cmds, board_visu, now, tile, pos);
    }
}

fn on_tile_click(
    cmds: &mut Commands,
    board_visu: &BoardVisu,
    now: IngameTimestamp,
    tile: &mut Tile,
    pos: &UVec2,
) {
    match tile {
        Tile::TowerGround(tower) => {
            if tower.is_none() {
                place_tower(cmds, tower, board_visu, pos, now);
            }
        }
        Tile::BuildingGround(_) => todo!(),
        _ => (),
    }
}

fn place_tower(
    cmds: &mut Commands,
    tower: &mut Option<Tower>,
    board_visu: &BoardVisu,
    pos: &UVec2,
    now: IngameTimestamp,
) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    let new_tower = Tower::laser_shot(pos, now);
    draw_tower(cmds, board_visu, pos, &new_tower, now);
    *tower = Some(new_tower);
}
