use bevy::{prelude::*, sprite::Anchor};

use crate::utils::TileResizeParams;

use super::ActionBoard;

#[derive(Component)]
struct BoardScreen;

pub(crate) fn draw_board(cmds: &mut Commands, board: &ActionBoard, start: Vec2, size: Vec2) {
    let rs_params = TileResizeParams::new(
        board.board(),
        start,
        Vec2::new(start.x + size.x, start.y + size.y),
    );

    // Board background
    cmds.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(size),
            color: Color::GRAY,
            anchor: Anchor::Center,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(start.x as f32, start.y as f32, 0.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(BoardScreen);

    // Road
    for posi in board.road_tile_posis() {
        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(rs_params.tile_size_vec),
                color: Color::DARK_GRAY,
                anchor: Anchor::Center,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(posi.x as f32, posi.y as f32, 0.1),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(BoardScreen);
    }
}
