use super::{
    actions::{
        build_menu::{
            BuildMenuBuildEvent, BuildMenuCloseEvent, BuildMenuHideEvent, BuildMenuOpenEvent,
            BuildMenuScrollEvent,
        },
        tile::TileActionsEvent,
        GameActionEvent,
    },
    build_menus::BuildMenu,
    GameScreen, HoveredTile, IngameState,
};
use crate::{
    board::{Board, Tile},
    utils::{cursor_pos, BoardPos, Vec2Board},
    CamQuery,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use TileActionsEvent::*;

type QueryPos<'w, 's, 'a> = Query<'w, 's, &'a BoardPos, With<GameScreen>>;

pub(super) fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut actions: EventWriter<GameActionEvent>,
    mut bm_scroll_ev: EventWriter<BuildMenuScrollEvent>,
    mut bm_build_ev: EventWriter<BuildMenuBuildEvent>,
    ingame_state: Res<State<IngameState>>,
) {
    use GameActionEvent::*;
    if keys.just_released(KeyCode::Escape) {
        actions.send(BackToMainMenu);
    }

    // Ingame keys
    if *ingame_state != IngameState::Running {
        return;
    }
    if keys.just_pressed(KeyCode::ShiftLeft) {
        actions.send(ActivateOverview);
    }
    if keys.just_released(KeyCode::ShiftLeft) {
        actions.send(DeactivateOverview);
    }
    if keys.just_pressed(KeyCode::Comma) {
        actions.send(SpeedDown)
    }
    if keys.just_pressed(KeyCode::Period) {
        actions.send(SpeedUp)
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
        bm_scroll_ev.send(BuildMenuScrollEvent::Before);
    }
    if keys.just_released(KeyCode::Down) {
        bm_scroll_ev.send(BuildMenuScrollEvent::After);
    }
    if keys.just_released(KeyCode::Return) {
        bm_build_ev.send(BuildMenuBuildEvent);
    }
}

pub(super) fn hovered_tile(
    mut current_tile: ResMut<HoveredTile>,
    wnds: Query<&Window>,
    board: Res<Board>,
    q_cam: CamQuery,
) {
    current_tile.0 = get_hover_pos_and_tile(wnds.single(), q_cam, board);
}

#[allow(clippy::too_many_arguments)]
pub(super) fn mouse_input(
    mbi: Res<Input<MouseButton>>,
    ev_scroll: EventReader<MouseWheel>,
    mut bm_open_ev: EventWriter<BuildMenuOpenEvent>,
    mut bm_close_ev: EventWriter<BuildMenuCloseEvent>,
    mut bm_scroll_ev: EventWriter<BuildMenuScrollEvent>,
    mut bm_build_ev: EventWriter<BuildMenuBuildEvent>,
    mut bm_hide_ev: EventWriter<BuildMenuHideEvent>,
    mut tile_ac: EventWriter<TileActionsEvent>,
    hovered_tile: Res<HoveredTile>,
    q_pos: QueryPos,
    tbm: Res<BuildMenu>,
) {
    match hovered_tile.0 {
        Some((pos, tile)) => tile_hover(
            &mut bm_open_ev,
            &mut bm_scroll_ev,
            &mut bm_build_ev,
            &mut bm_hide_ev,
            &mut tile_ac,
            ev_scroll,
            &mbi,
            tbm,
            q_pos,
            pos,
            tile,
        ),
        None => tile_unhover(&mut bm_hide_ev, &mut tile_ac),
    };

    if mbi.just_pressed(MouseButton::Right) {
        bm_close_ev.send(BuildMenuCloseEvent);
    }
}

#[allow(clippy::too_many_arguments)]
fn tile_hover(
    bm_open_ev: &mut EventWriter<BuildMenuOpenEvent>,
    bm_scroll_ev: &mut EventWriter<BuildMenuScrollEvent>,
    bm_build_ev: &mut EventWriter<BuildMenuBuildEvent>,
    bm_hide_ev: &mut EventWriter<BuildMenuHideEvent>,
    tile_acts: &mut EventWriter<TileActionsEvent>,
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
    match (is_left_click, tbm.is_open, is_tile_filled) {
        (true, true, false) => bm_build_ev.send(BuildMenuBuildEvent),
        (true, false, false) if is_build_tile => bm_open_ev.send(BuildMenuOpenEvent(upos)),
        (false, true, true) => bm_hide_ev.send(BuildMenuHideEvent),
        (false, true, _) if !is_build_tile => bm_hide_ev.send(BuildMenuHideEvent),
        (false, true, false) if tbm.should_open(upos) => bm_open_ev.send(BuildMenuOpenEvent(upos)),
        _ => (),
    }
    mouse_wheel_handler(ev_scroll, bm_open_ev, bm_scroll_ev, &tbm, &pos, &tile);
    tile_acts.send(HoverTile(pos));
}

fn tile_unhover(
    bm_hide_ev: &mut EventWriter<BuildMenuHideEvent>,
    tile_ac: &mut EventWriter<TileActionsEvent>,
) {
    tile_ac.send(UnhoverTile);
    bm_hide_ev.send(BuildMenuHideEvent);
}

fn mouse_wheel_handler(
    mut ev_scroll: EventReader<MouseWheel>,
    bm_open_ev: &mut EventWriter<BuildMenuOpenEvent>,
    bm_scroll_ev: &mut EventWriter<BuildMenuScrollEvent>,
    tbm: &BuildMenu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    for ev in ev_scroll.read() {
        match tile.is_buildable() {
            true if tbm.is_open => bm_scroll_ev.send(match ev.y > 0. {
                true => BuildMenuScrollEvent::Before,
                false => BuildMenuScrollEvent::After,
            }),
            true => bm_open_ev.send(BuildMenuOpenEvent(pos.as_uvec2())),
            _ => (),
        }
    }
}

fn get_hover_pos_and_tile(
    wnds: &Window,
    q_cam: CamQuery,
    board: Res<Board>,
) -> Option<(Vec2Board, Tile)> {
    if let Some(pos) = cursor_pos(wnds, q_cam) {
        if pos.x >= 0. && pos.y >= 0. {
            if let Some(tile) = board.get_tile(&pos.as_uvec2()) {
                return Some((pos, *tile));
            }
        }
    }
    None
}
