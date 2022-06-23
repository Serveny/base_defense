use super::BoardEditorScreen;
use crate::utils::{road_end_shape, TileResizeParams};
use bevy::prelude::*;

#[derive(Component)]
pub(super) struct BoardEditorRoadStartMark;

#[derive(Component)]
pub(super) struct BoardEditorRoadEndMark;

pub(super) fn spawn_end_marker(cmds: &mut Commands, rs_params: &TileResizeParams, pos: UVec2) {
    cmds.spawn_bundle(road_end_shape(
        rs_params.tile_inner_size.x,
        get_mark_translation(pos, rs_params),
    ))
    .insert(BoardEditorRoadEndMark)
    .insert(BoardEditorScreen);
}

fn get_mark_translation(pos: UVec2, rs_params: &TileResizeParams) -> Vec3 {
    let substract = rs_params.tile_inner_size.x / 2.;
    Vec3::new(
        rs_params.board_start_x + (pos.x as f32 * rs_params.tile_size) + substract,
        rs_params.board_start_y - (pos.y as f32 * rs_params.tile_size) - substract,
        1.,
    )
}
pub(super) fn pos_mark(pos: UVec2, trans: &mut Transform, rs_params: &TileResizeParams) {
    let substract = rs_params.tile_inner_size.x / 2.;
    trans.translation = Vec3::new(
        rs_params.board_start_x + (pos.x as f32 * rs_params.tile_size) + substract,
        rs_params.board_start_y - (pos.y as f32 * rs_params.tile_size) - substract,
        0.,
    );
}

pub(super) fn set_mark(
    commands: &mut Commands,
    mark: Option<(Entity, Mut<Transform>)>,
    pos: Option<UVec2>,
    rs_params: &TileResizeParams,
    is_start: bool,
) {
    if let Some(pos) = pos {
        if let Some(mut mark) = mark {
            pos_mark(pos, &mut mark.1, rs_params);
        } else if !is_start {
            spawn_end_marker(commands, rs_params, pos);
        }
    } else if let Some(mark) = mark {
        commands.entity(mark.0).despawn_recursive();
    }
}
