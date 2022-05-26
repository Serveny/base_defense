use crate::{
    assets::StandardAssets,
    board::{Board, Tile},
};
use bevy::prelude::*;

use super::{editor_tiles::TileResizeParams, BoardEditorScreen, BoardEditorState};

#[derive(Component)]
pub(super) struct EditorRoadStartPoint;

#[derive(Component)]
pub(super) struct EditorRoadEndPoint;

pub(super) fn spawn_markers(commands: &mut Commands, assets: &Res<StandardAssets>) {
    // Road start marker
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.editor_road_start.clone(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(EditorRoadStartPoint)
        .insert(BoardEditorScreen);

    // Road end marker
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.editor_road_end.clone(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(EditorRoadEndPoint)
        .insert(BoardEditorScreen);
}

pub(super) fn resize_markers(
    rs_params: &TileResizeParams,
    mut road_start: Query<&mut Transform, With<EditorRoadStartPoint>>,
    mut road_end: Query<&mut Transform, With<EditorRoadEndPoint>>,
    state: &Res<BoardEditorState>,
) {
    let board = &state.current_map;
    let road_tiles = board.get_tiles(Tile::Road);
    let building_tiles = board.get_tiles(Tile::BuildingGround(None));
    if let Some(start_pos) = board.get_road_start_pos(&road_tiles) {
        let mut transform = road_start.single_mut();
        transform.translation = Vec3::new(
            rs_params.board_start_x + (start_pos.x as f32 * rs_params.tile_size),
            rs_params.board_start_y - (start_pos.y as f32 * rs_params.tile_size),
            0.,
        );
    }
    if let Some(end_pos) = Board::get_road_end_pos(&road_tiles, &building_tiles) {
        let mut transform = road_end.single_mut();
        transform.translation = Vec3::new(
            rs_params.board_start_x + (end_pos.x as f32 * rs_params.tile_size),
            rs_params.board_start_y - (end_pos.y as f32 * rs_params.tile_size),
            0.,
        );
    }
}
