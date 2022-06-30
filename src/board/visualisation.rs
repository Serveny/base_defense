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

// Ingame Visualisation
pub struct BoardVisualisation<TScreen: Component + Clone> {
    // fm = from mid
    board_start_fm: Vec3,
    pub tile_size: f32,
    pub inner_tile_size: f32,
    tile_size_vec: Vec2,
    pub half_tile_vec3: Vec3,
    pub screen: TScreen,
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
impl<TScreen: Component + Copy> BoardVisualisation<TScreen> {
    pub fn new(
        win: &Window,
        board: &Board,
        margin_left: f32,
        margin_top: f32,
        tile_margin: f32,
        screen: TScreen,
    ) -> BoardVisualisation<TScreen> {
        let tile_size =
            Self::get_tile_size_px(win.width() - margin_left, win.height() - margin_top, board);
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
        // Board tiles
        for (y, row) in board.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                self.spawn_tile(cmds, Vec2Board::new(x as f32, y as f32), tile);
            }
        }

        // Road end mark
        spawn_road_end_mark(cmds, board_cache, self);
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
        .insert(BoardScreen)
        .insert(self.screen);
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
        query_hover_cross: &mut Query<(Entity, &mut Transform), With<BoardHoverCross>>,
        pos: &Vec2Board,
    ) {
        let translation = self.pos_to_px(Vec2Board::new(pos.x.floor(), pos.y.floor()), 1.);
        if let Ok(mut hover_cross) = query_hover_cross.get_single_mut() {
            hover_cross.1.translation = translation.into();
        } else {
            let shape = Self::hover_cross_shape(self.tile_size, translation);
            cmds.spawn_bundle(shape)
                .insert(BoardHoverCross)
                .insert(BoardScreen)
                .insert(self.screen);
        }
    }

    pub fn delete_hover_cross(
        cmds: &mut Commands,
        query_hover_cross: &mut Query<(Entity, &mut Transform), With<BoardHoverCross>>,
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

    pub fn set_road_end_mark(&self, mut query: RoadEndMarkQuery, board_cache: &BoardCache) {
        if let Some(end_pos) = board_cache.road_end_pos {
            if let Some(last_step) = board_cache.road_path.last() {
                query.for_each_mut(|(mut visi, mut transform, comp)| {
                    if !comp.is_child {
                        transform.translation =
                            self.pos_to_px(Vec2Board::from_uvec2_middle(&end_pos), 3.);
                        transform.rotation = Quat::from_rotation_z(
                            Angle::degrees(last_step.angle().to_degrees() + 180.).radians,
                        );
                    }
                    visi.is_visible = true;
                });
                return;
            }
        }
        query.for_each_mut(|(mut visi, _, _)| visi.is_visible = false);
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
    pub fn distance_board_to_px(&self, dist_board: f32) -> f32 {
        dist_board * self.tile_size
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
    fn get_tile_size_px(available_width_px: f32, available_height_px: f32, board: &Board) -> f32 {
        let tile_width_px = available_width_px / board.width as f32;
        let tile_height_px = available_height_px / board.height as f32;

        if tile_height_px > tile_width_px {
            tile_width_px
        } else {
            tile_height_px
        }
    }
}

mod road_end_mark {
    use bevy::prelude::*;
    use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
    use euclid::Angle;

    use super::{BoardRoadEndMark, BoardScreen, BoardVisualisation};
    use crate::{board::BoardCache, utils::Vec2Board};

    pub fn spawn_road_end_mark<TScreen: Component + Copy>(
        cmds: &mut Commands,
        board_cache: &BoardCache,
        board_visu: &BoardVisualisation<TScreen>,
    ) {
        let is_visible =
            board_cache.road_path.last().is_some() && board_cache.road_end_pos.is_some();
        let pos = board_cache.road_end_pos.unwrap_or_default();

        let angle = if let Some(last_step) = board_cache.road_path.last() {
            Angle::degrees(last_step.angle().to_degrees() + 180.)
        } else {
            Angle::default()
        };

        cmds.spawn_bundle(road_end_shape(
            board_visu.inner_tile_size,
            Transform {
                translation: board_visu
                    .pos_to_px_with_tile_margin(Vec2Board::from_uvec2_middle(&pos), 3.),
                rotation: Quat::from_rotation_z(angle.radians),
                ..Default::default()
            },
            is_visible,
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(road_end_entry_shape(board_visu.inner_tile_size, is_visible))
                .insert(BoardRoadEndMark::child())
                .insert(BoardScreen)
                .insert(board_visu.screen);
        })
        .insert(BoardRoadEndMark::parent())
        .insert(BoardScreen)
        .insert(board_visu.screen);
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
