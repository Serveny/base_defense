use self::road_end_mark::spawn_road_end_mark;
use super::Tile;
use crate::{
    assets::StandardAssets,
    board::{cache::BoardCache, Board},
    utils::Vec2Board,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use euclid::Angle;
use std::marker::PhantomData;

// Tile size factor, because bevy_lyon can't handle to small screen scales
pub const TILE_SIZE: f32 = 1000.;

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
    tile_size_vec: Vec2,
    pub half_tile_vec3: Vec3,
    screen: PhantomData<TScreen>,
}

#[derive(Component)]
pub struct BoardHoverCross;

#[derive(Component)]
pub struct BoardRoadEndMark {
    is_child: bool,
}

#[derive(Component)]
pub struct GameOverCountDownText;

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

impl<TScreen: Component + Default> BoardVisualisation<TScreen> {
    pub fn new(tile_scale: f32) -> BoardVisualisation<TScreen> {
        let inner_tile_size = tile_scale * TILE_SIZE;

        Self {
            inner_tile_size,
            tile_size_vec: Vec2::new(inner_tile_size, inner_tile_size),
            half_tile_vec3: Vec3::new(0.5 * TILE_SIZE, 0.5 * TILE_SIZE, 0.),
            screen: PhantomData,
        }
    }

    pub fn draw_board(
        &self,
        cmds: &mut Commands,
        board: &Board,
        board_cache: &BoardCache,
        assets: &StandardAssets,
    ) {
        // Board tiles
        for (y, row) in board.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                self.spawn_tile(cmds, Vec2Board::new(x as f32, y as f32), tile);
            }
        }

        // Road end mark
        self.spawn_road_end_mark(cmds, board_cache, assets);

        // Hover cross
        self.spawn_hover_cross(cmds);
    }

    fn spawn_tile(&self, cmds: &mut Commands, pos: Vec2Board, tile: &Tile) {
        cmds.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(self.tile_size_vec),
                color: Self::get_tile_color(tile),
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: pos.to_scaled_vec3(0.),
                ..Default::default()
            },

            ..Default::default()
        })
        .insert(BoardVisualTile::new(pos.as_uvec2()))
        .insert(BoardScreen)
        .insert(TScreen::default());
    }

    fn spawn_road_end_mark(
        &self,
        cmds: &mut Commands,
        board_cache: &BoardCache,
        assets: &StandardAssets,
    ) {
        spawn_road_end_mark::<TScreen>(cmds, board_cache, self.inner_tile_size, assets);
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
            Tile::TowerGround => Color::GOLD,
            Tile::BuildingGround => Color::ANTIQUE_WHITE,
            Tile::Road => Color::GRAY,
            Tile::Empty => Color::DARK_GRAY,
        }
    }

    pub fn show_hover_cross(&self, query: &mut HoverCrossQuery, pos: &Vec2Board) {
        let (mut visi, mut transform) = query.single_mut();
        transform.translation = Vec3::new(pos.x.floor() * TILE_SIZE, pos.y.ceil() * TILE_SIZE, 0.1);
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

    pub fn set_road_end_mark(&self, mut query: RoadEndMarkQuery, board_cache: &BoardCache) {
        if let Some(end_pos) = board_cache.road_end_pos {
            if let Some(last_step) = board_cache.road_path.last() {
                query.for_each_mut(|(mut visi, mut transform, comp)| {
                    if !comp.is_child {
                        transform.translation =
                            Vec2Board::from_uvec2_middle(&end_pos).to_scaled_vec3(3.);
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

    pub fn repaint(
        &self,
        cmds: &mut Commands,
        mut query: Query<Entity, With<BoardScreen>>,
        board: &Board,
        board_cache: &BoardCache,
        assets: &StandardAssets,
    ) {
        for entity in query.iter_mut() {
            cmds.entity(entity).despawn_recursive();
        }
        self.draw_board(cmds, board, board_cache, assets);
    }

    fn hover_cross_shape() -> ShapeBundle {
        GeometryBuilder::build_as(
            &Self::hover_cross_path(),
            DrawMode::Stroke(StrokeMode::new(Color::SILVER, TILE_SIZE / 8.)),
            Transform::default(),
        )
    }

    fn hover_cross_path() -> Path {
        let ts = TILE_SIZE;
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

    use super::{BoardRoadEndMark, BoardScreen, GameOverCountDownText, TILE_SIZE};
    use crate::{
        assets::StandardAssets,
        board::BoardCache,
        utils::{
            energy::{energy_symbol, EnergyText, ENERGY_COLOR},
            materials::{materials_symbol, MaterialsText, MATERIALS_COLOR},
            text_background_shape, text_bundle,
            wave::{wave_symbol, WaveText},
            Vec2Board,
        },
    };

    pub fn spawn_road_end_mark<TScreen: Component + Default>(
        cmds: &mut Commands,
        board_cache: &BoardCache,
        tile_size: f32,
        assets: &StandardAssets,
    ) {
        let is_visible =
            board_cache.road_path.last().is_some() && board_cache.road_end_pos.is_some();

        let angle = if let Some(last_step) = board_cache.road_path.last() {
            Angle::degrees(last_step.angle().to_degrees())
        } else {
            Angle::default()
        };
        let pos = board_cache.road_end_pos.unwrap_or_default();
        let pos_board = Vec2Board::from_uvec2_tilesize_middle(&pos, tile_size / TILE_SIZE);

        cmds.spawn_bundle(road_end_shape(
            tile_size,
            Transform {
                translation: pos_board.to_scaled_vec3(3.),
                scale: Vec3::new(2., 2., 1.),
                rotation: Quat::from_rotation_z(angle.radians),
            },
            is_visible,
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(road_end_entry_shape(tile_size, is_visible))
                .insert(BoardRoadEndMark::child());
        })
        .insert(BoardRoadEndMark::parent())
        .insert(BoardScreen)
        .insert(TScreen::default());

        let mut pos_wave_no = pos_board.to_scaled_vec3(3.1);
        pos_wave_no.y += tile_size / 5.;
        spawn_wave_sign::<TScreen>(cmds, assets, tile_size / 1.25, pos_wave_no);

        // Energy sign
        let pos_energy = pos_board.to_scaled_vec3(3.1);
        spawn_energy_sign::<TScreen>(cmds, assets, tile_size / 1.25, pos_energy);

        // Materials sign
        let mut pos_materials = pos_board.to_scaled_vec3(3.1);
        pos_materials.y -= tile_size / 5.;
        spawn_materials_sign::<TScreen>(cmds, assets, tile_size / 1.25, pos_materials);

        spawn_countdown_text::<TScreen>(
            cmds,
            assets,
            tile_size / 1.25,
            pos_board.to_scaled_vec3(3.2),
        );
    }

    fn spawn_countdown_text<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &StandardAssets,
        size: f32,
        translation: Vec3,
    ) {
        let mut bundle = text_bundle(
            size,
            "",
            Color::ORANGE_RED,
            assets,
            Transform::from_translation(translation),
            HorizontalAlign::Center,
        );
        bundle.visibility.is_visible = false;

        cmds.spawn_bundle(bundle)
            .insert(TScreen::default())
            .insert(GameOverCountDownText);
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
                fill_mode: FillMode::color(Color::OLIVE),
                outline_mode: StrokeMode::new(Color::DARK_GRAY, size_px / 10.),
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
                fill_mode: FillMode::color(Color::OLIVE),
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

    fn spawn_wave_sign<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &StandardAssets,
        width: f32,
        translation: Vec3,
    ) {
        cmds.spawn_bundle(text_background_shape(
            width,
            Transform {
                translation,
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            true,
        ))
        .insert(BoardScreen)
        .insert(TScreen::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(text_bundle(
                    width / 6.,
                    &format!("{}", 0),
                    Color::GOLD,
                    assets,
                    Transform::from_translation(Vec3::new(-width / 9., 0., 1.)),
                    HorizontalAlign::Left,
                ))
                .insert(WaveText);
            parent.spawn_bundle(wave_symbol(
                Transform {
                    translation: Vec3::new(-width / 6., 0., 0.),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                Color::GOLD,
            ));
        });
    }

    fn spawn_energy_sign<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &StandardAssets,
        width: f32,
        translation: Vec3,
    ) {
        cmds.spawn_bundle(text_background_shape(
            width,
            Transform {
                translation,
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            true,
        ))
        .insert(BoardScreen)
        .insert(TScreen::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(text_bundle(
                    width / 6.,
                    &format!("{}", 0),
                    ENERGY_COLOR,
                    assets,
                    Transform::from_translation(Vec3::new(-width / 9., 0., 1.)),
                    HorizontalAlign::Left,
                ))
                .insert(EnergyText);
            parent.spawn_bundle(energy_symbol(
                Transform {
                    translation: Vec3::new(-width / 6., 0., 0.),
                    scale: Vec3::new(0.1, 0.1, 1.),
                    ..Default::default()
                },
                ENERGY_COLOR,
            ));
        });
    }

    fn spawn_materials_sign<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &StandardAssets,
        width: f32,
        translation: Vec3,
    ) {
        cmds.spawn_bundle(text_background_shape(
            width,
            Transform {
                translation,
                scale: Vec3::new(2., 2., 1.),
                ..Default::default()
            },
            true,
        ))
        .insert(BoardScreen)
        .insert(TScreen::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(text_bundle(
                    width / 6.,
                    &format!("{}", 0),
                    MATERIALS_COLOR,
                    assets,
                    Transform::from_translation(Vec3::new(-width / 9., 0., 1.)),
                    HorizontalAlign::Left,
                ))
                .insert(MaterialsText);
            parent.spawn_bundle(materials_symbol(
                Transform {
                    translation: Vec3::new(-width / 6., 0., 0.),
                    scale: Vec3::new(0.1, 0.1, 1.),
                    ..Default::default()
                },
                MATERIALS_COLOR,
            ));
        });
    }
}
