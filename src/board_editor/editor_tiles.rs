use crate::board::{Board, Tile};
use bevy::{prelude::*, sprite::Anchor};

use super::{
    actions::{cursor_pos_to_transform_pos, is_hover},
    BoardEditorScreen, BoardEditorState, LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX,
};

#[derive(Component, Debug)]
pub(super) struct EditorTile {
    pub pos: UVec2,
}

impl EditorTile {
    fn new(pos: UVec2) -> Self {
        Self { pos }
    }
}

pub(super) struct TileResizeParams {
    pub tile_size: f32,
    pub tile_inner_size: Vec2,
    pub board_start_x: f32,
    pub board_start_y: f32,
}

impl TileResizeParams {
    pub fn new(windows: &Windows, board: &Board) -> Self {
        let window = windows.get_primary().unwrap();
        let board_width_px = window.width() - LEFT_BAR_WIDTH_PX;
        let board_height_px = window.height() - TOP_BAR_HEIGHT_PX;

        // the tiles are quadratic, so use the smaller size
        let tile_size = get_tile_size_px(board_width_px, board_height_px, board);
        let tile_inner_size = Vec2::new(tile_size - 10., tile_size - 10.);

        Self {
            tile_size,
            tile_inner_size,

            // Think from the middle of the sceen
            board_start_x: (LEFT_BAR_WIDTH_PX - board_width_px) / 2.,
            board_start_y: (board_height_px - TOP_BAR_HEIGHT_PX) / 2.,
        }
    }
}

pub(super) fn spawn_tiles(commands: &mut Commands, windows: &Windows, board: &mut Board) {
    let rs_params = TileResizeParams::new(windows, board);
    for (y, row) in board.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            spawn_tile(tile, x, y, &rs_params, commands);
        }
    }
}

fn spawn_tile(
    tile: &Tile,
    x: usize,
    y: usize,
    rs_params: &TileResizeParams,
    commands: &mut Commands,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(rs_params.tile_inner_size),
                color: get_tile_color(tile),
                anchor: Anchor::TopLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(
                    rs_params.board_start_x + (x as f32 * rs_params.tile_size),
                    rs_params.board_start_y - (y as f32 * rs_params.tile_size),
                    0.,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EditorTile::new(UVec2::new(x as u32, y as u32)))
        .insert(BoardEditorScreen);
}

fn get_tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::TowerGround(_) => Color::GOLD,
        Tile::BuildingGround(_) => Color::ANTIQUE_WHITE,
        Tile::Road => Color::AQUAMARINE,
        Tile::Empty => Color::DARK_GRAY,
    }
}

fn get_tile_size_px(board_width_px: f32, board_height_px: f32, board: &Board) -> f32 {
    let tile_width_px = board_width_px / board.width as f32;
    let tile_height_px = board_height_px / board.height as f32;

    if tile_height_px > tile_width_px {
        tile_width_px
    } else {
        tile_height_px
    }
}

pub(super) fn set_tile(
    windows: Res<Windows>,
    mut state: ResMut<BoardEditorState>,
    mut editor_tiles: Query<(&mut Sprite, &Transform, &EditorTile), With<EditorTile>>,
    tile_to: Tile,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor_pos) = window.cursor_position() {
        for (mut sprite, transform, tile) in editor_tiles.iter_mut() {
            if is_hover(
                cursor_pos_to_transform_pos(cursor_pos, &window),
                &sprite,
                &transform,
            ) {
                if state.current_map.tiles[tile.pos.y as usize][tile.pos.x as usize] != tile_to {
                    sprite.color = get_tile_color(&tile_to);
                    state.current_map.tiles[tile.pos.y as usize][tile.pos.x as usize] = tile_to;
                    state.err_text = match state.current_map.validate() {
                        Ok(()) => None,
                        Err(err) => Some(String::from(err)),
                    }
                }

                break;
            }
        }
    }
}

pub(super) fn edit_board_size(board: &mut Board, new_width: u8, new_heigth: u8) {
    // Add/reduce width
    if new_width > board.width {
        let to_add = new_width - board.width;
        for row in &mut board.tiles {
            for _ in 0..to_add {
                row.push(Tile::Empty);
            }
        }
    } else if new_width < board.width {
        let to_del = board.width - new_width;
        for row in &mut board.tiles {
            for _ in 0..to_del {
                row.pop();
            }
        }
    }

    // Add/reduce height
    if new_heigth > board.height {
        let to_add = new_heigth - board.height;
        for _ in 0..to_add {
            let mut row = Vec::new();
            for _ in 0..board.width {
                row.push(Tile::Empty);
            }
            board.tiles.push(row);
        }
    } else if new_heigth < board.height {
        let to_del = board.height - new_heigth;
        for _ in 0..to_del {
            board.tiles.pop();
        }
    }

    board.width = new_width;
    board.height = new_heigth;
}

pub(super) fn resize_tiles(
    rs_params: &TileResizeParams,
    mut editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
) {
    editor_tiles.for_each_mut(|(mut sprite, mut transform, tile)| {
        sprite.custom_size = Some(rs_params.tile_inner_size);
        transform.translation = Vec3::new(
            rs_params.board_start_x + (tile.pos.x as f32 * rs_params.tile_size),
            rs_params.board_start_y - (tile.pos.y as f32 * rs_params.tile_size),
            0.,
        );
    });
}
