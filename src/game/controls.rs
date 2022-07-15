use super::{
    actions::{build_menu::BuildMenuActionsEvent, tile::TileActionsEvent, GameActionEvent},
    build_menus::BuildMenu,
};
use crate::{
    board::{Board, Tile},
    utils::{cursor_pos, Vec2Board},
    CamQuery,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use BuildMenuActionsEvent::*;
use TileActionsEvent::*;

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

#[allow(clippy::too_many_arguments)]
pub(super) fn mouse_input(
    mut tile_ac: EventWriter<TileActionsEvent>,
    mut tm_ac: EventWriter<BuildMenuActionsEvent>,
    ev_scroll: EventReader<MouseWheel>,
    mbi: Res<Input<MouseButton>>,
    q_cam: CamQuery,
    wnds: Res<Windows>,
    board: Res<Board>,
    tbm: Res<BuildMenu>,
) {
    match get_hover_pos_and_tile(wnds, q_cam, board) {
        Some((pos, tile)) => tile_hover(&mut tile_ac, &mut tm_ac, ev_scroll, &mbi, tbm, pos, tile),
        None => tile_unhover(&mut tile_ac, &mut tm_ac),
    };

    if mbi.just_pressed(MouseButton::Right) {
        tm_ac.send(Close);
    }
}

fn tile_hover(
    tile_acts: &mut EventWriter<TileActionsEvent>,
    tm_acts: &mut EventWriter<BuildMenuActionsEvent>,
    ev_scroll: EventReader<MouseWheel>,
    mbi: &Input<MouseButton>,
    tbm: Res<BuildMenu>,
    pos: Vec2Board,
    tile: Tile,
) {
    use Tile::*;
    let upos = pos.as_uvec2();
    if let Some(ev) = match (mbi.just_pressed(MouseButton::Left), tbm.is_open, &tile) {
        (true, true, _) => Some(BuildMenuActionsEvent::Build),
        (true, false, _) => Some(Open(upos)),
        (false, true, _) if tbm.tile_pos != upos => Some(Open(upos)),
        (false, true, TowerGround(None) | BuildingGround(None)) => Some(Open(upos)),
        (false, true, _) => Some(Close),
        (false, false, _) => None,
    } {
        tm_acts.send(ev);
    }
    mouse_wheel_handler(tm_acts, ev_scroll, &tbm, &pos, &tile);
    tile_acts.send(HoverTile(pos));
}

fn tile_unhover(
    tile_ac: &mut EventWriter<TileActionsEvent>,
    tm_ac: &mut EventWriter<BuildMenuActionsEvent>,
) {
    tile_ac.send(UnhoverTile);
    tm_ac.send(Close);
}

fn mouse_wheel_handler(
    tm_actions: &mut EventWriter<BuildMenuActionsEvent>,
    mut ev_scroll: EventReader<MouseWheel>,
    tbm: &BuildMenu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    for ev in ev_scroll.iter() {
        // println!("{:?}", ev);
        match (tile, tbm.is_open) {
            (Tile::TowerGround(None) | Tile::BuildingGround(None), true) => {
                send_tbm_scroll_ev(ev, tm_actions)
            }
            (Tile::TowerGround(None) | Tile::BuildingGround(None), false) => {
                tm_actions.send(Open(pos.as_uvec2()))
            }
            _ => (),
        }
    }
}

fn send_tbm_scroll_ev(ev: &MouseWheel, tm_actions: &mut EventWriter<BuildMenuActionsEvent>) {
    tm_actions.send(if ev.y > 0. { ScollUp } else { ScollDown });
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
