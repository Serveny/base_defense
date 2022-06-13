use bevy::{prelude::*, sprite::Anchor};

use super::ActionBoard;

#[derive(Component)]
struct BoardScreen;

pub(crate) fn draw_board(cmds: &mut Commands, board: &ActionBoard, start: UVec2, size: Vec2) {
    todo!();
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
}
