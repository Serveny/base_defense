use self::road_end_mark::spawn_road_end_mark;
use super::Tile;
use crate::{
    board::{cache::BoardCache, Board},
    utils::{towers::TowerRangeCircle, Vec2Board},
};
use bevy::{
    color::palettes::css::{ANTIQUE_WHITE, DARK_GRAY, DIM_GRAY, GOLD, GRAY},
    prelude::*,
    sprite::Anchor,
};
use bevy_prototype_lyon::prelude::*;
use euclid::Angle;
use std::marker::PhantomData;

// Tile size factor, because bevy_lyon can't handle to small screen scales
pub const TILE_SIZE: f32 = 1000.;

pub type QueryBoardVisuTile<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Sprite, &'a Transform, &'a BoardVisualTile),
    (With<BoardVisualTile>, Without<BoardRoadEndMark>),
>;
pub type RoadEndMarkQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform, &'a BoardRoadEndMark),
    With<BoardRoadEndMark>,
>;
pub type HoverCrossQuery<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform),
    (With<BoardHoverCross>, Without<TowerRangeCircle>),
>;

// Ingame Visualisationtile_size
#[derive(Resource)]
pub struct BoardVisualisation<TScreen> {
    pub inner_tile_size: f32,
    tile_size_vec: Vec2,
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
            screen: PhantomData,
        }
    }

    pub fn draw_board(
        &self,
        cmds: &mut Commands,
        board: &Board,
        board_cache: &BoardCache,
        assets: &AssetServer,
    ) {
        // Board tiles
        for (y, row) in board.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                self.spawn_tile(cmds, Vec2Board::new(x as f32, y as f32), *tile);
            }
        }

        // Road end mark
        self.spawn_road_end_mark(cmds, board_cache, assets);

        // Hover cross
        self.spawn_hover_cross(cmds);
    }

    fn spawn_tile(&self, cmds: &mut Commands, pos: Vec2Board, tile: Tile) {
        cmds.spawn((
            Sprite {
                custom_size: Some(self.tile_size_vec),
                color: Self::get_tile_color(tile),
                ..Default::default()
            },
            Anchor::BOTTOM_LEFT,
            Transform {
                translation: pos.to_scaled_vec3(0.),
                ..Default::default()
            },
        ))
        .insert(BoardVisualTile::new(pos.as_uvec2()))
        .insert(BoardScreen)
        .insert(TScreen::default());
    }

    fn spawn_road_end_mark(
        &self,
        cmds: &mut Commands,
        board_cache: &BoardCache,
        assets: &AssetServer,
    ) {
        spawn_road_end_mark::<TScreen>(cmds, board_cache, self.inner_tile_size, assets);
    }

    pub fn change_tile(pos: UVec2, to: Tile, q_board_visu_tile: &mut QueryBoardVisuTile) {
        for (mut sprite, _, vis_tile) in q_board_visu_tile.iter_mut() {
            if vis_tile.pos == pos {
                sprite.color = Self::get_tile_color(to);
                break;
            }
        }
    }

    fn get_tile_color(tile: Tile) -> Color {
        match tile {
            Tile::TowerGround => GOLD,
            Tile::BuildingGround => ANTIQUE_WHITE,
            Tile::Road => GRAY,
            Tile::Empty => DARK_GRAY,
        }
        .into()
    }

    pub fn show_hover_cross(&self, query: &mut HoverCrossQuery, pos: &Vec2Board) {
        let Ok((mut visi, mut transform)) = query.single_mut() else {
            return;
        };
        transform.translation = Vec3::new(pos.x.floor() * TILE_SIZE, pos.y.ceil() * TILE_SIZE, 0.1);
        *visi = Visibility::Visible;
    }

    pub fn hide_hover_cross(query: &mut HoverCrossQuery) {
        if let Ok(mut cross) = query.single_mut() {
            *cross.0 = Visibility::Hidden;
        }
    }

    fn spawn_hover_cross(&self, cmds: &mut Commands) {
        cmds.spawn(Self::hover_cross_shape())
            .insert(BoardHoverCross)
            .insert(BoardScreen)
            .insert(TScreen::default());
    }

    pub fn set_road_end_mark(&self, query: &mut RoadEndMarkQuery, board_cache: &BoardCache) {
        if let Some(end_pos) = board_cache.road_end_pos {
            if let Some(last_step) = board_cache.road_path.last() {
                query
                    .iter_mut()
                    .for_each(|(mut visi, mut transform, comp)| {
                        if !comp.is_child {
                            transform.translation =
                                Vec2Board::from_uvec2_middle(&end_pos).to_scaled_vec3(3.);
                            transform.rotation = Quat::from_rotation_z(
                                Angle::degrees(last_step.angle().to_degrees()).radians,
                            );
                        }
                        *visi = Visibility::Visible;
                    });
                return;
            }
        }
        query
            .iter_mut()
            .for_each(|(mut visi, _, _)| *visi = Visibility::Hidden);
    }

    pub fn repaint(
        &self,
        cmds: &mut Commands,
        query: &Query<Entity, With<BoardScreen>>,
        board: &Board,
        board_cache: &BoardCache,
        assets: &AssetServer,
    ) {
        for board_screen_id in query.iter() {
            cmds.entity(board_screen_id).try_despawn();
        }
        self.draw_board(cmds, board, board_cache, assets);
    }

    fn hover_cross_shape() -> impl Bundle {
        (
            ShapeBuilder::with(&Self::hover_cross_path())
                .fill(Color::srgba(1., 1., 1., 0.05))
                .stroke(Stroke::new(DIM_GRAY, TILE_SIZE / 8.))
                .build(),
            Visibility::Hidden,
        )
    }

    fn hover_cross_path() -> ShapePath {
        let ts = TILE_SIZE;
        let eighth = ts / 8.;
        let one_third = ts / 3.;

        ShapePath::new()
            // top left
            .move_to(Vec2::new(eighth, -one_third))
            .line_to(Vec2::new(eighth, -eighth))
            .line_to(Vec2::new(one_third, -eighth))
            // top right
            .move_to(Vec2::new(ts - eighth, -one_third))
            .line_to(Vec2::new(ts - eighth, -eighth))
            .line_to(Vec2::new(ts - one_third, -eighth))
            // bottom right
            .move_to(Vec2::new(ts - eighth, -ts + one_third))
            .line_to(Vec2::new(ts - eighth, -ts + eighth))
            .line_to(Vec2::new(ts - one_third, -ts + eighth))
            // bottom left
            .move_to(Vec2::new(eighth, -ts + one_third))
            .line_to(Vec2::new(eighth, -ts + eighth))
            .line_to(Vec2::new(one_third, -ts + eighth))
    }
}

mod road_end_mark {
    use bevy::prelude::*;
    use bevy_prototype_lyon::prelude::*;
    use euclid::Angle;

    use super::{BoardRoadEndMark, BoardScreen, GameOverCountDownText, TILE_SIZE};
    use crate::{
        board::BoardCache,
        utils::{
            energy::{energy_symbol, EnergyText, ENERGY_COLOR},
            materials::{materials_symbol, MaterialsText, MATERIALS_COLOR},
            text_bundle, text_bundle_with_anchor, visible,
            wave::{wave_symbol, WaveText},
            Vec2Board,
        },
    };
    use bevy::color::palettes::css::{DIM_GRAY, GOLD, OLIVE, ORANGE_RED};
    use bevy::sprite::Anchor;

    pub fn spawn_road_end_mark<TScreen: Component + Default>(
        cmds: &mut Commands,
        board_cache: &BoardCache,
        tile_size: f32,
        assets: &AssetServer,
    ) {
        let visibility =
            visible(board_cache.road_path.last().is_some() && board_cache.road_end_pos.is_some());

        let angle = if let Some(last_step) = board_cache.road_path.last() {
            Angle::degrees(last_step.angle().to_degrees())
        } else {
            Angle::default()
        };
        let pos = board_cache.road_end_pos.unwrap_or_default();
        let pos_board = Vec2Board::from_uvec2_tilesize_middle(&pos, tile_size / TILE_SIZE);

        cmds.spawn(road_end_shape(
            tile_size,
            Transform {
                translation: pos_board.to_scaled_vec3(3.),
                scale: Vec3::new(2., 2., 1.),
                rotation: Quat::from_rotation_z(angle.radians),
            },
            visibility,
        ))
        .with_children(|parent| {
            parent
                .spawn(road_end_entry_shape(tile_size, visibility))
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
        assets: &AssetServer,
        size: f32,
        translation: Vec3,
    ) {
        let bundle = text_bundle("", ORANGE_RED.into(), assets, translation, size / 2.);

        cmds.spawn(bundle)
            .insert(Visibility::Hidden)
            .insert(BoardScreen)
            .insert(TScreen::default())
            .insert(GameOverCountDownText);
    }

    fn road_end_shape(size_px: f32, transform: Transform, visibility: Visibility) -> impl Bundle {
        (
            ShapeBuilder::with(&shapes::RegularPolygon {
                sides: 8,
                feature: shapes::RegularPolygonFeature::Radius(size_px / 3.),
                ..default()
            })
            .fill(OLIVE)
            .stroke(Stroke::new(DIM_GRAY, size_px / 10.))
            .build(),
            transform,
            visibility,
        )
    }

    fn road_end_entry_shape(size_px: f32, visibility: Visibility) -> impl Bundle {
        (
            ShapeBuilder::with(&shapes::Rectangle {
                origin: RectangleOrigin::Center,
                extents: Vec2::new(size_px / 4., size_px / 2.),
                radii: None,
            })
            .fill(OLIVE)
            .stroke(Stroke::new(DIM_GRAY, size_px / 32.))
            .build(),
            Transform::from_translation(Vec3::new(size_px / 3., 0., -0.1)),
            visibility,
        )
    }

    fn spawn_wave_sign<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &AssetServer,
        width: f32,
        translation: Vec3,
    ) {
        let (symbol_translation, text_translation) = sign_translations(translation, width);
        cmds.spawn(text_bundle_with_anchor(
            &format!("{}", 0),
            GOLD.into(),
            assets,
            text_translation,
            width / 4.5,
            Anchor::CENTER_LEFT,
        ))
        .insert(WaveText)
        .insert(BoardScreen)
        .insert(TScreen::default());
        cmds.spawn(wave_symbol(
            Transform {
                translation: symbol_translation,
                scale: Vec3::new(1., 1., 1.),
                ..Default::default()
            },
            GOLD.into(),
        ))
        .insert(BoardScreen)
        .insert(TScreen::default());
    }

    fn spawn_energy_sign<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &AssetServer,
        width: f32,
        translation: Vec3,
    ) {
        let (symbol_translation, text_translation) = sign_translations(translation, width);
        cmds.spawn(text_bundle_with_anchor(
            &format!("{}", 0),
            ENERGY_COLOR.into(),
            assets,
            text_translation,
            width / 4.5,
            Anchor::CENTER_LEFT,
        ))
        .insert(EnergyText)
        .insert(BoardScreen)
        .insert(TScreen::default());
        cmds.spawn(energy_symbol(
            Transform {
                translation: symbol_translation,
                scale: Vec3::new(0.1, 0.1, 1.),
                ..Default::default()
            },
            ENERGY_COLOR.into(),
        ))
        .insert(BoardScreen)
        .insert(TScreen::default());
    }

    fn spawn_materials_sign<TScreen: Component + Default>(
        cmds: &mut Commands,
        assets: &AssetServer,
        width: f32,
        translation: Vec3,
    ) {
        let (symbol_translation, text_translation) = sign_translations(translation, width);
        cmds.spawn(text_bundle_with_anchor(
            &format!("{}", 0),
            MATERIALS_COLOR.into(),
            assets,
            text_translation,
            width / 4.5,
            Anchor::CENTER_LEFT,
        ))
        .insert(BoardScreen)
        .insert(TScreen::default())
        .insert(MaterialsText);
        cmds.spawn(materials_symbol(
            Transform {
                translation: symbol_translation,
                scale: Vec3::new(0.1, 0.1, 1.),
                ..Default::default()
            },
            MATERIALS_COLOR.into(),
        ))
        .insert(BoardScreen)
        .insert(TScreen::default());
    }

    fn sign_translations(mut center: Vec3, width: f32) -> (Vec3, Vec3) {
        let mut symbol = center;
        symbol.x -= width / 7.;
        center.x += width / 12.;
        (symbol, center)
    }
}
