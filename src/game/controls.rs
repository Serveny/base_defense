use super::{
    actions::{tile::TileActionsEvent, GameActionEvent},
    tower_build_menu::TowerBuildMenu,
};
use crate::{
    board::{Board, Tile},
    utils::{cursor_pos, Vec2Board},
    CamQuery,
};
use bevy::{input::mouse::MouseWheel, prelude::*};

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
    mut actions: EventWriter<TileActionsEvent>,
    mut ev_scroll: EventReader<MouseWheel>,
    mouse_button_input: Res<Input<MouseButton>>,
    q_cam: CamQuery,
    wnds: Res<Windows>,
    board: Res<Board>,
    tbm: Res<TowerBuildMenu>,
) {
    if let Some((pos, tile)) = get_hover_pos_and_tile(wnds, q_cam, board) {
        if mouse_button_input.pressed(MouseButton::Left) {
            actions.send(TileActionsEvent::TileLeftClick(pos.as_uvec2()));
        }
        if tbm.is_open && tbm.tile_pos != pos.as_uvec2() {
            actions.send(if let Tile::TowerGround(_) = tile {
                TileActionsEvent::OpenTowerBuildMenu(pos.as_uvec2())
            } else {
                TileActionsEvent::CloseTowerBuildMenu
            });
        }

        for ev in ev_scroll.iter() {
            println!("{:?}", ev);
            if let Tile::TowerGround(tile) = &tile {
                if tile.is_none() {
                    if tbm.is_open {
                        send_tbm_scroll_ev(ev, &mut actions);
                    } else {
                        actions.send(TileActionsEvent::OpenTowerBuildMenu(pos.as_uvec2()));
                    }
                }
            }
        }

        actions.send(TileActionsEvent::HoverTile(pos));
    } else {
        actions.send(TileActionsEvent::UnhoverTile);
        actions.send(TileActionsEvent::CloseTowerBuildMenu);
    }
}

fn send_tbm_scroll_ev(ev: &MouseWheel, actions: &mut EventWriter<TileActionsEvent>) {
    actions.send(if ev.y > 0. {
        TileActionsEvent::ScollUpTowerBuildMenu
    } else {
        TileActionsEvent::ScollDownTowerBuildMenu
    });
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
