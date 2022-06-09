use crate::{utils::TileResizeParams, BACKGROUND_COLOR};

use super::BoardEditorScreen;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub(super) struct BoardEditorRoadStartMark;

#[derive(Component)]
pub(super) struct BoardEditorRoadEndMark;

pub(super) fn spawn_start_marker(
    commands: &mut Commands,
    rs_params: &TileResizeParams,
    pos: UVec2,
) {
    let shape = shapes::RegularPolygon {
        sides: 8,
        feature: shapes::RegularPolygonFeature::Radius(rs_params.tile_inner_size.x / 4.),
        ..shapes::RegularPolygon::default()
    };
    let line_width = rs_params.tile_size - rs_params.tile_inner_size.x;

    // Road start marker
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::CYAN),
                outline_mode: StrokeMode::new(BACKGROUND_COLOR, line_width),
            },
            get_mark_transform(pos, rs_params),
        ))
        .insert(BoardEditorRoadStartMark)
        .insert(BoardEditorScreen);
}

pub(super) fn spawn_end_marker(commands: &mut Commands, rs_params: &TileResizeParams, pos: UVec2) {
    let shape = shapes::RegularPolygon {
        sides: 8,
        feature: shapes::RegularPolygonFeature::Radius(rs_params.tile_inner_size.x / 4.),
        ..shapes::RegularPolygon::default()
    };
    let line_width = rs_params.tile_size - rs_params.tile_inner_size.x;

    // Road end marker
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::SEA_GREEN),
                outline_mode: StrokeMode::new(BACKGROUND_COLOR, line_width),
            },
            get_mark_transform(pos, rs_params),
        ))
        .insert(BoardEditorRoadEndMark)
        .insert(BoardEditorScreen);
}

fn get_mark_transform(pos: UVec2, rs_params: &TileResizeParams) -> Transform {
    let substract = rs_params.tile_inner_size.x / 2.;
    Transform {
        translation: Vec3::new(
            rs_params.board_start_x + (pos.x as f32 * rs_params.tile_size) + substract,
            rs_params.board_start_y - (pos.y as f32 * rs_params.tile_size) - substract,
            0.,
        ),
        ..Default::default()
    }
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
        } else {
            if is_start {
                spawn_start_marker(commands, rs_params, pos)
            } else {
                spawn_end_marker(commands, rs_params, pos)
            }
        }
    } else if let Some(mark) = mark {
        commands.entity(mark.0).despawn_recursive();
    }
}
