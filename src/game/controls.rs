use super::{
    actions::{tile::TileActionsEvent, tower_menu::TowerMenuActionsEvent, GameActionEvent},
    tower_build_menu::TowerMenu,
};
use crate::{
    board::{Board, Tile},
    utils::{cursor_pos, Vec2Board},
    CamQuery,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use TileActionsEvent::*;
use TowerMenuActionsEvent::*;

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
    mut tile_actions: EventWriter<TileActionsEvent>,
    mut tm_actions: EventWriter<TowerMenuActionsEvent>,
    ev_scroll: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_cam: CamQuery,
    wnds: Res<Windows>,
    board: Res<Board>,
    tbm: Res<TowerMenu>,
) {
    if let Some((pos, tile)) = get_hover_pos_and_tile(wnds, q_cam, board) {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            if tbm.is_open {
                tm_actions.send(TowerMenuActionsEvent::PlaceTower);
                tm_actions.send(Close);
            } else {
                tm_actions.send(Open(pos.as_uvec2()));
            }
        } else {
            if tbm.is_open && tbm.tile_pos != pos.as_uvec2() {
                tm_actions.send(if let Tile::TowerGround(_) = tile {
                    Open(pos.as_uvec2())
                } else {
                    Close
                });
            }
            mouse_wheel_handler(&mut tm_actions, ev_scroll, &tbm, &pos, &tile);
        }
        tile_actions.send(HoverTile(pos));
    } else {
        tile_actions.send(UnhoverTile);
        tm_actions.send(Close);
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        tm_actions.send(Close);
    }
}

fn mouse_wheel_handler(
    tm_actions: &mut EventWriter<TowerMenuActionsEvent>,
    mut ev_scroll: EventReader<MouseWheel>,
    tbm: &TowerMenu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    for ev in ev_scroll.iter() {
        // println!("{:?}", ev);
        if let Tile::TowerGround(tile) = &tile {
            if tile.is_none() {
                if tbm.is_open {
                    send_tbm_scroll_ev(ev, tm_actions);
                } else {
                    tm_actions.send(Open(pos.as_uvec2()));
                }
            }
        }
    }
}

fn send_tbm_scroll_ev(ev: &MouseWheel, tm_actions: &mut EventWriter<TowerMenuActionsEvent>) {
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
