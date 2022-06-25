use crate::{
    board::{cache::BoardCache, Board},
    utils::{get_tile_size_px, road_end_shape, Vec2Board},
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

use super::Tile;

// Ingame Visualisation
pub struct BoardVisualisation<TScreen: Component + Clone> {
    // fm = from mid
    board_start_fm: Vec3,
    pub tile_size: f32,
    inner_tile_size: f32,
    tile_size_vec: Vec2,
    pub half_tile_vec3: Vec3,
    screen: TScreen,
}

#[derive(Component)]
pub struct BoardHoverCross;

#[derive(Component)]
pub struct BoardRoadEndMark;

#[derive(Component, Debug)]
pub struct BoardVisualTile {
    pub pos: UVec2,
}

impl BoardVisualTile {
    fn new(pos: UVec2) -> Self {
        Self { pos }
    }
}
impl<TScreen: Component + Clone> BoardVisualisation<TScreen> {
    pub fn new(
        win: &Window,
        board: &Board,
        margin_left: f32,
        margin_top: f32,
        tile_margin: f32,
        screen: TScreen,
    ) -> BoardVisualisation<TScreen> {
        let tile_size =
            get_tile_size_px(win.width() - margin_left, win.height() - margin_top, board);
        let inner_tile_size = tile_size - (tile_margin * 2.);
        let board_size = Vec2::new(
            tile_size * board.width as f32,
            tile_size * board.height as f32,
        );
        BoardVisualisation {
            board_start_fm: Vec3::new(
                (-board_size.x + margin_left) / 2.,
                (board_size.y - margin_top) / 2.,
                0.,
            ),
            tile_size,
            inner_tile_size,
            tile_size_vec: Vec2::new(inner_tile_size, inner_tile_size),
            half_tile_vec3: Vec3::new(tile_size / 2., -tile_size / 2., 0.),
            screen,
        }
    }

    pub fn draw_board(&self, cmds: &mut Commands, board: &Board, board_cache: &BoardCache) {
        for (y, row) in board.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                self.spawn_tile(cmds, Vec2Board::new(x as f32, y as f32), tile);
            }
        }
        self.set_road_end_mark(cmds, board_cache.road_end_pos, None);
    }

    fn spawn_tile(&self, cmds: &mut Commands, pos: Vec2Board, tile: &Tile) {
        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(self.tile_size_vec),
                color: Self::get_tile_color(tile),
                anchor: Anchor::TopLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: self.pos_to_px(pos, 0.).into(),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(BoardVisualTile::new(pos.as_uvec2()))
        .insert(self.screen.clone());
    }

    pub fn repaint(
        &self,
        cmds: &mut Commands,
        mut query: Query<Entity, With<TScreen>>,
        board: &Board,
        board_cache: &BoardCache,
    ) {
        for entity in query.iter_mut() {
            cmds.entity(entity).despawn_recursive();
        }
        self.draw_board(cmds, board, board_cache);
    }

    pub fn set_road_end_mark(
        &self,
        cmds: &mut Commands,
        pos: Option<UVec2>,
        mark_query: Option<Query<(Entity, &mut Transform), With<BoardRoadEndMark>>>,
    ) {
        if let Some(pos) = pos {
            self.pos_road_end_mark(cmds, mark_query, pos);
        } else {
            Self::despawn_road_end_mark(cmds, mark_query);
        }
    }

    fn pos_road_end_mark(
        &self,
        cmds: &mut Commands,
        mark: Option<Query<(Entity, &mut Transform), With<BoardRoadEndMark>>>,
        pos: UVec2,
    ) {
        if let Some(mut mark) = mark {
            if let Ok(mut mark) = mark.get_single_mut() {
                mark.1.translation =
                    self.pos_to_px_with_tile_margin(Vec2Board::from_uvec2_middle(&pos), 1.)
            } else {
                self.spawn_road_end_mark(cmds, pos);
            }
        } else {
            self.spawn_road_end_mark(cmds, pos);
        }
    }

    fn spawn_road_end_mark(&self, cmds: &mut Commands, pos: UVec2) {
        cmds.spawn_bundle(road_end_shape(
            self.inner_tile_size,
            self.pos_to_px_with_tile_margin(Vec2Board::from_uvec2_middle(&pos), 1.),
        ))
        .insert(self.screen.clone());
    }

    fn despawn_road_end_mark(
        cmds: &mut Commands,
        mut mark: Option<Query<(Entity, &mut Transform), With<BoardRoadEndMark>>>,
    ) {
        if let Some(mark) = mark.as_mut() {
            if let Ok(mark) = mark.get_single_mut() {
                cmds.entity(mark.0).despawn_recursive();
            }
        }
    }

    pub fn change_tile(
        pos: &UVec2,
        to: &Tile,
        mut query: Query<(&mut Sprite, &Transform, &BoardVisualTile), With<BoardVisualTile>>,
    ) {
        for (mut sprite, _, vis_tile) in query.iter_mut() {
            if vis_tile.pos == *pos {
                sprite.color = Self::get_tile_color(&to);
                break;
            }
        }
    }

    fn get_tile_color(tile: &Tile) -> Color {
        match tile {
            Tile::TowerGround(_) => Color::GOLD,
            Tile::BuildingGround(_) => Color::ANTIQUE_WHITE,
            Tile::Road => Color::GRAY,
            Tile::Empty => Color::DARK_GRAY,
        }
    }

    pub fn draw_hover_cross(
        &self,
        cmds: &mut Commands,
        mut query_hover_cross: Query<(Entity, &mut Transform), With<BoardHoverCross>>,
        pos: Vec2Board,
    ) {
        let translation = self.pos_to_px(Vec2Board::new(pos.x.floor(), pos.y.floor()), 1.);
        if let Ok(mut hover_cross) = query_hover_cross.get_single_mut() {
            hover_cross.1.translation = translation.into();
        } else {
            let shape = Self::hover_cross_shape(self.tile_size, translation);
            cmds.spawn_bundle(shape)
                .insert(BoardHoverCross)
                .insert(self.screen.clone());
        }
    }

    pub fn delete_hover_cross(
        cmds: &mut Commands,
        mut query_hover_cross: Query<(Entity, &mut Transform), With<BoardHoverCross>>,
    ) {
        if let Ok(hover_cross) = query_hover_cross.get_single_mut() {
            cmds.entity(hover_cross.0).despawn_recursive();
        }
    }

    pub fn get_hover_pos(&self, win: &Window) -> Option<Vec2Board> {
        if let Some(cursor_pos) = win.cursor_position() {
            return Some(self.cursor_px_to_board_pos(Vec2::new(
                cursor_pos.x - win.width() / 2.,
                cursor_pos.y - win.height() / 2.,
            )));
        }
        None
    }

    pub fn pos_to_px(&self, pos: Vec2Board, z: f32) -> Vec3 {
        Vec3::new(
            self.board_start_fm.x + (pos.x * self.tile_size),
            self.board_start_fm.y - (pos.y * self.tile_size),
            z,
        )
    }
    pub fn pos_to_px_with_tile_margin(&self, pos: Vec2Board, z: f32) -> Vec3 {
        let half_margin = (self.tile_size - self.inner_tile_size) / 2.;
        let mut pos_px = self.pos_to_px(pos, z);
        pos_px.x -= half_margin;
        pos_px.y += half_margin;
        pos_px
    }

    // pub fn distance_board_to_px(&self, distance_board: Vec2Board) -> Vec3 {
    //     Vec3::new(
    //         distance_board.x * self.tile_size,
    //         distance_board.y * self.tile_size,
    //         0.,
    //     )
    // }

    pub fn cursor_px_to_board_pos(&self, cursor_pos_px: Vec2) -> Vec2Board {
        Vec2Board::new(
            (cursor_pos_px.x - self.board_start_fm.x) / self.tile_size,
            (self.board_start_fm.y - cursor_pos_px.y) / self.tile_size,
        )
    }

    fn hover_cross_shape(tile_size: f32, translation: Vec3) -> ShapeBundle {
        GeometryBuilder::build_as(
            &Self::hover_cross_path(tile_size),
            DrawMode::Stroke(StrokeMode::new(Color::SILVER, tile_size / 8.)),
            Transform {
                translation,
                ..Default::default()
            },
        )
    }

    fn hover_cross_path(tile_size: f32) -> Path {
        let ts = tile_size;
        let eighth = ts / 8.;
        let one_third = ts / 3.;
        let mut pb = PathBuilder::new();

        // top left
        pb.move_to(Vec2::new(eighth, -one_third));
        pb.line_to(Vec2::new(eighth, -eighth));
        pb.line_to(Vec2::new(one_third, -eighth));

        // top right
        pb.move_to(Vec2::new(ts - eighth, -one_third));
        pb.line_to(Vec2::new(ts - eighth, -eighth));
        pb.line_to(Vec2::new(ts - one_third, -eighth));

        // bottom right
        pb.move_to(Vec2::new(ts - eighth, -ts + one_third));
        pb.line_to(Vec2::new(ts - eighth, -ts + eighth));
        pb.line_to(Vec2::new(ts - one_third, -ts + eighth));

        // bottom left
        pb.move_to(Vec2::new(eighth, -ts + one_third));
        pb.line_to(Vec2::new(eighth, -ts + eighth));
        pb.line_to(Vec2::new(one_third, -ts + eighth));

        pb.build()
    }
}
