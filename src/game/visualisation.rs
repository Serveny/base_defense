use super::{ActionBoard, GameScreen};
use crate::utils::{get_tile_color, get_tile_size_px};
use bevy::{prelude::*, sprite::Anchor};
use bevy_prototype_lyon::prelude::*;

#[derive(Debug, Clone)]
pub(super) struct Visualisation {
    // fm = from mid
    board_start_fm: Vec2,
    tile_size: f32,
    tile_size_vec: Vec2,
}

#[derive(Component)]
pub(super) struct HoverCross;

impl Visualisation {
    pub fn new(win: &Window, action_board: &ActionBoard, margin_top: f32) -> Visualisation {
        let board = action_board.board();
        let tile_size = get_tile_size_px(win.width(), win.height(), board);
        let board_size = Vec2::new(
            tile_size * board.width as f32,
            tile_size * board.height as f32,
        );
        Visualisation {
            board_start_fm: Vec2::new(-board_size.x / 2., (board_size.y - margin_top) / 2.),
            tile_size,
            tile_size_vec: Vec2::new(tile_size, tile_size),
        }
    }

    pub fn draw_board(&self, cmds: &mut Commands, action_board: &ActionBoard) {
        for (y, row) in action_board.board().tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                cmds.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(self.tile_size_vec),
                        color: get_tile_color(tile),
                        anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: self.pos_to_px(Vec2::new(x as f32, y as f32), 0.),
                        ..Default::default()
                    },

                    ..Default::default()
                })
                .insert(GameScreen);
            }
        }
    }

    pub fn draw_hover_cross(
        &self,
        cmds: &mut Commands,
        mut query_hover_cross: Query<(Entity, &mut Transform), With<HoverCross>>,
        pos: Vec2,
    ) {
        let translation = self.pos_to_px(Vec2::new(pos.x.floor(), pos.y.floor()), 1.);

        if let Ok(mut hover_cross_trans) = query_hover_cross.get_single_mut() {
            hover_cross_trans.1.translation = translation;
        } else {
            let shape = GeometryBuilder::build_as(
                &self.hover_cross_path(),
                DrawMode::Stroke(StrokeMode::new(Color::SILVER, 10.0)),
                Transform {
                    translation,
                    ..Default::default()
                },
            );

            cmds.spawn_bundle(shape).insert(HoverCross);
        }
    }

    pub fn delete_hover_cross(
        cmds: &mut Commands,
        mut query_hover_cross: Query<(Entity, &mut Transform), With<HoverCross>>,
    ) {
        if let Ok(hover_cross) = query_hover_cross.get_single_mut() {
            cmds.entity(hover_cross.0).despawn_recursive();
        }
    }

    fn pos_to_px(&self, pos: Vec2, z: f32) -> Vec3 {
        Vec3::new(
            self.board_start_fm.x + (pos.x * self.tile_size),
            self.board_start_fm.y - (pos.y * self.tile_size),
            z,
        )
    }

    pub fn cursor_px_to_board_pos(&self, cursor_pos_px: Vec2) -> Vec2 {
        Vec2::new(
            (cursor_pos_px.x - self.board_start_fm.x) / self.tile_size,
            (self.board_start_fm.y - cursor_pos_px.y) / self.tile_size,
        )
    }

    fn hover_cross_path(&self) -> Path {
        let ts = self.tile_size;
        let eighth = ts / 8.;
        let one_third = ts / 3.;
        let mut path_builder = PathBuilder::new();

        // top left
        path_builder.move_to(Vec2::new(eighth, -one_third));
        path_builder.line_to(Vec2::new(eighth, -eighth));
        path_builder.line_to(Vec2::new(one_third, -eighth));

        // top right
        path_builder.move_to(Vec2::new(ts - eighth, -one_third));
        path_builder.line_to(Vec2::new(ts - eighth, -eighth));
        path_builder.line_to(Vec2::new(ts - one_third, -eighth));

        // bottom right
        path_builder.move_to(Vec2::new(ts - eighth, -ts + one_third));
        path_builder.line_to(Vec2::new(ts - eighth, -ts + eighth));
        path_builder.line_to(Vec2::new(ts - one_third, -ts + eighth));

        // bottom left
        path_builder.move_to(Vec2::new(eighth, -ts + one_third));
        path_builder.line_to(Vec2::new(eighth, -ts + eighth));
        path_builder.line_to(Vec2::new(one_third, -ts + eighth));

        path_builder.build()
    }
}
