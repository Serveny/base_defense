use bevy::prelude::*;

use crate::{
    board::{visualisation::HoverCrossQuery, Board, Tile},
    game::{
        tower_build_menu::{TowerBuildMenu, TowerBuildMenuComp},
        BoardVisu, Game, GameScreen,
    },
    utils::{
        towers::{draw_tower, Tower},
        Vec2Board,
    },
};

use super::{set_range_circles, RangeCircleQuery};

type TileActionQueries<'w, 's, 'a> = ParamSet<
    'w,
    's,
    (
        HoverCrossQuery<'w, 's, 'a>,
        RangeCircleQuery<'w, 's, 'a>,
        TowerBuildMenuQuery<'w, 's, 'a>,
    ),
>;
type TowerBuildMenuQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a mut Transform), With<TowerBuildMenuComp>>;

pub enum TileActionsEvent {
    HoverTile(Vec2Board),
    UnhoverTile,
    TileLeftClick(UVec2),
    OpenTowerBuildMenu(UVec2),
    ScollUpTowerBuildMenu,
    ScollDownTowerBuildMenu,
    CloseTowerBuildMenu,
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn on_tile_actions(
    mut cmds: Commands,
    mut actions: EventReader<TileActionsEvent>,
    mut board: ResMut<Board>,
    mut queries: TileActionQueries,
    mut tbm: ResMut<TowerBuildMenu>,
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
                TileActionsEvent::TileLeftClick(pos) => handle_click(&mut cmds, &mut board, pos),
                TileActionsEvent::OpenTowerBuildMenu(pos) => open_tbm(&mut tbm, queries.p2(), pos),
                TileActionsEvent::CloseTowerBuildMenu => close_tbm(&mut tbm, queries.p2()),
                TileActionsEvent::ScollUpTowerBuildMenu => todo!(),
                TileActionsEvent::ScollDownTowerBuildMenu => todo!(),
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

fn handle_click(cmds: &mut Commands, board: &mut Board, pos: &UVec2) {
    if let Some(tile) = board.get_tile_mut(pos) {
        on_tile_click(cmds, tile, pos);
    }
}

fn on_tile_click(cmds: &mut Commands, tile: &mut Tile, pos: &UVec2) {
    match tile {
        Tile::TowerGround(tower) => {
            if tower.is_none() {
                place_tower(cmds, tower, pos);
            }
        }
        Tile::BuildingGround(_) => todo!(),
        _ => (),
    }
}

fn place_tower(cmds: &mut Commands, tower: &mut Option<Tower>, pos: &UVec2) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    let new_tower = Tower::laser(pos);
    draw_tower::<GameScreen>(cmds, pos, &new_tower);
    *tower = Some(new_tower);
}

fn open_tbm(tbm: &mut TowerBuildMenu, q_tbm: TowerBuildMenuQuery, pos: &UVec2) {
    set_visibility_tbm(q_tbm, true, Vec2Board::from_uvec2_middle(pos));
    tbm.tile_pos = *pos;
    tbm.is_open = true;
}

fn close_tbm(tbm: &mut TowerBuildMenu, q_tbm: TowerBuildMenuQuery) {
    set_visibility_tbm(q_tbm, false, Vec2Board::default());
    tbm.is_open = false;
}

fn set_visibility_tbm(mut q_tbm: TowerBuildMenuQuery, is_visible: bool, pos: Vec2Board) {
    q_tbm.for_each_mut(|(mut visi, mut transform)| {
        transform.translation = pos.to_scaled_vec3(3.);
        visi.is_visible = is_visible;
    });
}
