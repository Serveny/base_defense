use super::{
    actions::{build_menu::BuildMenuActionsEvent, tile::TileActionsEvent, GameActionEvent},
    build_menus::BuildMenu,
    GameScreen, HoveredTile, IngameState,
};
use crate::{
    board::{Board, Tile},
    utils::{cursor_pos, BoardPos, Vec2Board},
    CamQuery,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use BuildMenuActionsEvent::*;
use TileActionsEvent::*;

type QueryPos<'w, 's, 'a> = Query<'w, 's, &'a BoardPos, With<GameScreen>>;

pub(super) fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut actions: EventWriter<GameActionEvent>,
    mut tm_actions: EventWriter<BuildMenuActionsEvent>,
    ingame_state: Res<State<IngameState>>,
) {
    use GameActionEvent::*;
    if keys.just_released(KeyCode::Escape) {
        actions.send(BackToMainMenu);
    }

    // Ingame keys
    if *ingame_state.current() != IngameState::Running {
        return;
    }
    if keys.just_pressed(KeyCode::LShift) {
        actions.send(ActivateOverview);
    }
    if keys.just_released(KeyCode::LShift) {
        actions.send(DeactivateOverview);
    }
    if keys.just_pressed(KeyCode::F) {
        actions.send(Speed(4.))
    }
    if keys.just_released(KeyCode::F) {
        actions.send(Speed(1.))
    }
    if keys.just_released(KeyCode::P) {
        actions.send(Pause)
    }

    // Build Menu
    if keys.just_released(KeyCode::Up) {
        tm_actions.send(BuildMenuActionsEvent::EntryBefore);
    }
    if keys.just_released(KeyCode::Down) {
        tm_actions.send(BuildMenuActionsEvent::EntryAfter);
    }
    if keys.just_released(KeyCode::Return) {
        tm_actions.send(BuildMenuActionsEvent::Build);
    }
}

pub(super) fn hovered_tile(
    mut current_tile: ResMut<HoveredTile>,
    wnds: Res<Windows>,
    board: Res<Board>,
    q_cam: CamQuery,
) {
    current_tile.0 = get_hover_pos_and_tile(wnds, q_cam, board);
}

pub(super) fn mouse_input(
    mut tile_ac: EventWriter<TileActionsEvent>,
    mut tm_ac: EventWriter<BuildMenuActionsEvent>,
    hovered_tile: Res<HoveredTile>,
    ev_scroll: EventReader<MouseWheel>,
    mbi: Res<Input<MouseButton>>,
    q_pos: QueryPos,
    tbm: Res<BuildMenu>,
) {
    match hovered_tile.0.clone() {
        Some((pos, tile)) => tile_hover(
            &mut tile_ac,
            &mut tm_ac,
            ev_scroll,
            &mbi,
            tbm,
            q_pos,
            pos,
            tile,
        ),
        None => tile_unhover(&mut tile_ac, &mut tm_ac),
    };

    if mbi.just_pressed(MouseButton::Right) {
        tm_ac.send(Close);
    }
}

#[allow(clippy::too_many_arguments)]
fn tile_hover(
    tile_acts: &mut EventWriter<TileActionsEvent>,
    tm_acts: &mut EventWriter<BuildMenuActionsEvent>,
    ev_scroll: EventReader<MouseWheel>,
    mbi: &Input<MouseButton>,
    tbm: Res<BuildMenu>,
    p_pos: QueryPos,
    pos: Vec2Board,
    tile: Tile,
) {
    let upos = pos.as_uvec2();
    let is_left_click = mbi.just_pressed(MouseButton::Left);
    let is_tile_filled = p_pos.iter().any(|t_pos| upos == **t_pos);
    let is_build_tile = tile.is_buildable();
    if let Some(ev) = match (is_left_click, tbm.is_open, is_tile_filled) {
        (true, true, false) => Some(BuildMenuActionsEvent::Build),
        (true, false, false) if is_build_tile => Some(Open(upos)),
        (false, true, true) => Some(Hide),
        (false, true, _) if !is_build_tile => Some(Hide),
        (false, true, false) if tbm.should_open(upos) => Some(Open(upos)),
        _ => None,
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
    tm_ac.send(Hide);
}

fn mouse_wheel_handler(
    tm_actions: &mut EventWriter<BuildMenuActionsEvent>,
    mut ev_scroll: EventReader<MouseWheel>,
    tbm: &BuildMenu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    for ev in ev_scroll.iter() {
        match tile.is_buildable() {
            true if tbm.is_open => send_tbm_scroll_ev(ev, tm_actions),
            true => tm_actions.send(Open(pos.as_uvec2())),
            _ => (),
        }
    }
}

fn send_tbm_scroll_ev(ev: &MouseWheel, tm_actions: &mut EventWriter<BuildMenuActionsEvent>) {
    tm_actions.send(if ev.y > 0. { EntryBefore } else { EntryAfter });
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
