use super::{
    actions::{
        build_menu::{
            BuildMenuBuildMessage, BuildMenuCloseMessage, BuildMenuHideMessage,
            BuildMenuOpenMessage, BuildMenuScrollMessage,
        },
        tile::TileActionsMessage,
        GameActionMessage,
    },
    build_menus::BuildMenu,
    GameScreen, HoveredTile, IngameState,
};
use crate::{
    board::{Board, Tile},
    controls::{
        ControlAction, BUILD_MENU_MOUSE_BUTTON, CLOSE_BUILD_MENU_MOUSE_BUTTON, KEY_BINDINGS,
    },
    utils::{cursor_pos, BoardPos, Vec2Board},
    CamQuery,
};
use bevy::{input::mouse::MouseWheel, prelude::*};
use TileActionsMessage::*;

type QueryPos<'w, 's, 'a> = Query<'w, 's, &'a BoardPos, With<GameScreen>>;

pub(super) fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut actions: MessageWriter<GameActionMessage>,
    mut bm_scroll_ev: MessageWriter<BuildMenuScrollMessage>,
    mut bm_build_ev: MessageWriter<BuildMenuBuildMessage>,
    ingame_state: Res<State<IngameState>>,
) {
    for binding in KEY_BINDINGS.iter() {
        if keys.just_pressed(binding.key_code) {
            if let Some(control_action) = binding.on_press {
                write_control_action(
                    control_action,
                    &ingame_state,
                    &mut actions,
                    &mut bm_scroll_ev,
                    &mut bm_build_ev,
                );
            }
        }

        if keys.just_released(binding.key_code) {
            if let Some(control_action) = binding.on_release {
                write_control_action(
                    control_action,
                    &ingame_state,
                    &mut actions,
                    &mut bm_scroll_ev,
                    &mut bm_build_ev,
                );
            }
        }
    }
}

fn write_control_action(
    control_action: ControlAction,
    ingame_state: &State<IngameState>,
    actions: &mut MessageWriter<GameActionMessage>,
    bm_scroll_ev: &mut MessageWriter<BuildMenuScrollMessage>,
    bm_build_ev: &mut MessageWriter<BuildMenuBuildMessage>,
) {
    use GameActionMessage::*;

    match control_action {
        ControlAction::BackToMainMenu => {
            actions.write(BackToMainMenu);
        }
        _ if **ingame_state != IngameState::Running => (),
        ControlAction::ActivateOverview => {
            actions.write(ActivateOverview);
        }
        ControlAction::DeactivateOverview => {
            actions.write(DeactivateOverview);
        }
        ControlAction::SpeedDown => {
            actions.write(SpeedDown);
        }
        ControlAction::SpeedUp => {
            actions.write(SpeedUp);
        }
        ControlAction::FastForward => {
            actions.write(Speed(4.));
        }
        ControlAction::NormalSpeed => {
            actions.write(Speed(1.));
        }
        ControlAction::Pause => {
            actions.write(Pause);
        }
        ControlAction::BuildMenuPrevious => {
            bm_scroll_ev.write(BuildMenuScrollMessage::Before);
        }
        ControlAction::BuildMenuNext => {
            bm_scroll_ev.write(BuildMenuScrollMessage::After);
        }
        ControlAction::BuildSelected => {
            bm_build_ev.write(BuildMenuBuildMessage);
        }
    }
}

pub(super) fn hovered_tile(
    mut current_tile: ResMut<HoveredTile>,
    q_win: Query<&Window>,
    board: Res<Board>,
    q_cam: CamQuery,
) {
    current_tile.0 = get_hover_pos_and_tile(q_win, q_cam, board);
}

#[allow(clippy::too_many_arguments)]
pub(super) fn mouse_input(
    mbi: Res<ButtonInput<MouseButton>>,
    ev_scroll: MessageReader<MouseWheel>,
    mut bm_open_ev: MessageWriter<BuildMenuOpenMessage>,
    mut bm_close_ev: MessageWriter<BuildMenuCloseMessage>,
    mut bm_scroll_ev: MessageWriter<BuildMenuScrollMessage>,
    mut bm_build_ev: MessageWriter<BuildMenuBuildMessage>,
    mut bm_hide_ev: MessageWriter<BuildMenuHideMessage>,
    mut tile_ac: MessageWriter<TileActionsMessage>,
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

    if mbi.just_pressed(CLOSE_BUILD_MENU_MOUSE_BUTTON) {
        bm_close_ev.write(BuildMenuCloseMessage);
    }
}

#[allow(clippy::too_many_arguments)]
fn tile_hover(
    bm_open_ev: &mut MessageWriter<BuildMenuOpenMessage>,
    bm_scroll_ev: &mut MessageWriter<BuildMenuScrollMessage>,
    bm_build_ev: &mut MessageWriter<BuildMenuBuildMessage>,
    bm_hide_ev: &mut MessageWriter<BuildMenuHideMessage>,
    tile_acts: &mut MessageWriter<TileActionsMessage>,
    ev_scroll: MessageReader<MouseWheel>,
    mbi: &ButtonInput<MouseButton>,
    tbm: Res<BuildMenu>,
    p_pos: QueryPos,
    pos: Vec2Board,
    tile: Tile,
) {
    let upos = pos.as_uvec2();
    let is_left_click = mbi.just_pressed(BUILD_MENU_MOUSE_BUTTON);
    let is_tile_filled = p_pos.iter().any(|t_pos| upos == **t_pos);
    let is_build_tile = tile.is_buildable();
    match (is_left_click, tbm.is_open, is_tile_filled) {
        (true, true, false) => {
            bm_build_ev.write(BuildMenuBuildMessage);
        }
        (true, false, false) if is_build_tile => {
            bm_open_ev.write(BuildMenuOpenMessage(upos));
        }
        (false, true, true) => {
            bm_hide_ev.write(BuildMenuHideMessage);
        }
        (false, true, _) if !is_build_tile => {
            bm_hide_ev.write(BuildMenuHideMessage);
        }
        (false, true, false) if tbm.should_open(upos) => {
            bm_open_ev.write(BuildMenuOpenMessage(upos));
        }
        _ => (),
    };
    mouse_wheel_handler(ev_scroll, bm_open_ev, bm_scroll_ev, &tbm, &pos, &tile);
    tile_acts.write(HoverTile(pos));
}

fn tile_unhover(
    bm_hide_ev: &mut MessageWriter<BuildMenuHideMessage>,
    tile_ac: &mut MessageWriter<TileActionsMessage>,
) {
    tile_ac.write(UnhoverTile);
    bm_hide_ev.write(BuildMenuHideMessage);
}

fn mouse_wheel_handler(
    mut ev_scroll: MessageReader<MouseWheel>,
    bm_open_ev: &mut MessageWriter<BuildMenuOpenMessage>,
    bm_scroll_ev: &mut MessageWriter<BuildMenuScrollMessage>,
    tbm: &BuildMenu,
    pos: &Vec2Board,
    tile: &Tile,
) {
    for ev in ev_scroll.read() {
        match tile.is_buildable() {
            true if tbm.is_open => {
                bm_scroll_ev.write(match ev.y > 0. {
                    true => BuildMenuScrollMessage::Before,
                    false => BuildMenuScrollMessage::After,
                });
            }
            true => {
                bm_open_ev.write(BuildMenuOpenMessage(pos.as_uvec2()));
            }
            _ => (),
        }
    }
}

fn get_hover_pos_and_tile(
    q_win: Query<&Window>,
    q_cam: CamQuery,
    board: Res<Board>,
) -> Option<(Vec2Board, Tile)> {
    if let Some(pos) = cursor_pos(q_win, q_cam) {
        if pos.x >= 0. && pos.y >= 0. {
            if let Some(tile) = board.get_tile(&pos.as_uvec2()) {
                return Some((pos, *tile));
            }
        }
    }
    None
}
