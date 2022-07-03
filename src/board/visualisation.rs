use std::marker::PhantomData;

use self::road_end_mark::spawn_road_end_mark;
use super::Tile;
use crate::{
    board::{cache::BoardCache, Board},
    utils::Vec2Board,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use euclid::Angle;

pub type BoardScreenQuery<'a> = Query<'a, 'a, Entity, With<BoardScreen>>;
pub type RoadEndMarkQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform, &'a BoardRoadEndMark),
    With<BoardRoadEndMark>,
>;
pub type HoverCrossQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a mut Transform), With<BoardHoverCross>>;

// Ingame Visualisationtile_size
pub struct BoardVisualisation<TScreen> {
    pub inner_tile_size: f32,
    tile_size_vec: Vec2Board,
    pub half_tile_vec3: Vec3,
    screen: PhantomData<TScreen>,
}

#[derive(Component)]
pub struct BoardHoverCross;

#[derive(Component)]
pub struct BoardRoadEndMark {
    is_child: bool,
}

impl BoardRoadEndMark {
    pub fn child() -> Self {
        Self { is_child: true }
    }
    pub fn parent() -> Self {
        Self { is_child: false }
    }
}

#[derive(Component, Clone, Copy)]
pub struct BoardScreen;

#[derive(Component, Debug)]
pub struct BoardVisualTile {
    pub pos: UVec2,
}

impl BoardVisualTile {
    fn new(pos: UVec2) -> Self {
        Self { pos }
    }
}

impl<TScreen: Component + Copy + Default> BoardVisualisation<TScreen> {
    pub fn new(tile_scale: f32) -> BoardVisualisation<TScreen> {
        let inner_tile_size = tile_scale;

        Self {
            inner_tile_size,
            tile_size_vec: Vec2Board::new(inner_tile_size, inner_tile_size),
            half_tile_vec3: Vec3::new(0.5, 0.5, 0.),
            screen: PhantomData,
        }
    }

    pub fn draw_board(&self, cmds: &mut Commands, board: &Board, board_cache: &BoardCache) {
        // Board tiles
        for (y, row) in board.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                self.spawn_tile(cmds, Vec2Board::new(x as f32, y as f32), tile);
            }
        }

        // Road end mark
        self.spawn_road_end_mark(cmds, board_cache);

        // Hover cross
        self.spawn_hover_cross(cmds);
    }

    fn spawn_tile(&self, cmds: &mut Commands, pos: Vec2Board, tile: &Tile) {
        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(self.tile_size_vec.into()),
                color: Self::get_tile_color(tile),
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: pos.to_vec3(0.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(BoardVisualTile::new(pos.as_uvec2()))
        .insert(BoardScreen)
        .insert(TScreen::default());
    }

    fn spawn_road_end_mark(&self, cmds: &mut Commands, board_cache: &BoardCache) {
        spawn_road_end_mark(cmds, board_cache, self.inner_tile_size, TScreen::default());
    }

    pub fn change_tile(
        pos: &UVec2,
        to: &Tile,
        mut query: Query<(&mut Sprite, &Transform, &BoardVisualTile), With<BoardVisualTile>>,
    ) {
        for (mut sprite, _, vis_tile) in query.iter_mut() {
            if vis_tile.pos == *pos {
                sprite.color = Self::get_tile_color(to);
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

    pub fn show_hover_cross(&self, query: &mut HoverCrossQuery, pos: &Vec2Board) {
        let (mut visi, mut transform) = query.single_mut();
        transform.translation = Vec2Board::new(pos.x.floor(), pos.y.floor()).to_vec3(2.);
        visi.is_visible = true;
    }

    pub fn hide_hover_cross(query: &mut HoverCrossQuery) {
        query.single_mut().0.is_visible = false;
    }
    fn spawn_hover_cross(&self, cmds: &mut Commands) {
        let mut shape = Self::hover_cross_shape();
        shape.visibility.is_visible = false;
        cmds.spawn_bundle(shape)
            .insert(BoardHoverCross)
            .insert(BoardScreen)
            .insert(TScreen::default());
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

    pub fn set_road_end_mark(&self, mut query: RoadEndMarkQuery, board_cache: &BoardCache) {
        if let Some(end_pos) = board_cache.road_end_pos {
            if let Some(last_step) = board_cache.road_path.last() {
                query.for_each_mut(|(mut visi, mut transform, comp)| {
                    if !comp.is_child {
                        transform.translation = Vec2Board::from_uvec2_middle(&end_pos).to_vec3(3.);
                        transform.rotation = Quat::from_rotation_z(
                            Angle::degrees(last_step.angle().to_degrees()).radians,
                        );
                    }
                    visi.is_visible = true;
                });
                return;
            }
        }
        query.for_each_mut(|(mut visi, _, _)| visi.is_visible = false);
    }
    pub fn cursor_px_to_board_pos(&self, cursor_pos_px: Vec2) -> Vec2Board {
        Vec2Board::new(cursor_pos_px.x, cursor_pos_px.y)
    }

    pub fn repaint(
        &self,
        cmds: &mut Commands,
        mut query: Query<Entity, With<BoardScreen>>,
        board: &Board,
        board_cache: &BoardCache,
    ) {
        for entity in query.iter_mut() {
            cmds.entity(entity).despawn_recursive();
        }
        self.draw_board(cmds, board, board_cache);
    }

    fn hover_cross_shape() -> ShapeBundle {
        GeometryBuilder::build_as(
            &Self::hover_cross_path(),
            DrawMode::Stroke(StrokeMode::new(Color::SILVER, 1. / 8.)),
            Transform::default(),
        )
    }

    fn hover_cross_path() -> Path {
        let ts = 1.;
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

mod road_end_mark {
    use bevy::prelude::*;
    use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
    use euclid::Angle;

    use super::{BoardRoadEndMark, BoardScreen};
    use crate::board::BoardCache;

    pub fn spawn_road_end_mark<TScreen: Component + Copy>(
        cmds: &mut Commands,
        board_cache: &BoardCache,
        tile_size: f32,
        screen: TScreen,
    ) {
        let is_visible =
            board_cache.road_path.last().is_some() && board_cache.road_end_pos.is_some();

        let angle = if let Some(last_step) = board_cache.road_path.last() {
            Angle::degrees(last_step.angle().to_degrees())
        } else {
            Angle::default()
        };
        let pos = board_cache.road_end_pos.unwrap_or_default();

        cmds.spawn_bundle(road_end_shape(
            tile_size,
            Transform {
                translation: Vec3::new(pos.x as f32 + 0.5, pos.y as f32 + 0.5, 3.),
                rotation: Quat::from_rotation_z(angle.radians),
                ..Default::default()
            },
            is_visible,
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(road_end_entry_shape(tile_size, is_visible))
                .insert(BoardRoadEndMark::child())
                .insert(BoardScreen)
                .insert(screen);
        })
        .insert(BoardRoadEndMark::parent())
        .insert(BoardScreen)
        .insert(screen);
    }

    fn road_end_shape(size_px: f32, transform: Transform, is_visible: bool) -> ShapeBundle {
        let shape = shapes::RegularPolygon {
            sides: 8,
            feature: shapes::RegularPolygonFeature::Radius(size_px / 3.),
            ..shapes::RegularPolygon::default()
        };

        let mut shape_bundle = GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::SEA_GREEN),
                outline_mode: StrokeMode::new(Color::DARK_GRAY, size_px / 8.),
            },
            transform,
        );
        shape_bundle.visibility = Visibility { is_visible };
        shape_bundle
    }

    fn road_end_entry_shape(size_px: f32, is_visible: bool) -> ShapeBundle {
        let shape = shapes::Rectangle {
            origin: RectangleOrigin::Center,
            extents: Vec2::new(size_px / 4., size_px / 2.),
        };

        let mut shape_bundle = GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::SEA_GREEN),
                outline_mode: StrokeMode::new(Color::DARK_GRAY, size_px / 32.),
            },
            Transform {
                translation: Vec3::new(size_px / 3., 0., -0.1),
                ..Default::default()
            },
        );
        shape_bundle.visibility.is_visible = is_visible;
        shape_bundle
    }
}
