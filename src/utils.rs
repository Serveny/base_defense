#![allow(unused)]
use crate::board::{ActionBoard, Board, Tile};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use serde::{Deserialize, Serialize};

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
    MapEditor,
}

#[derive(strum::EnumIter, strum::Display, PartialEq, Eq, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Middle,
    Hard,
}

pub type Energy = u32;
pub type Materials = u32;

#[derive(Default, Deref, DerefMut, Clone, Copy, PartialEq, Debug)]
pub struct Vec2Board(Vec2);

impl Vec2Board {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
    pub fn from_uvec2_middle(uvec2: &UVec2) -> Self {
        Self::new(uvec2.x as f32 + 0.5, uvec2.y as f32 + 0.5)
    }
}

impl From<Vec2> for Vec2Board {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
    }
}
impl From<UVec2> for Vec2Board {
    fn from(uvec2: UVec2) -> Self {
        Self(uvec2.as_vec2())
    }
}

impl Add for Vec2Board {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Mul for Vec2Board {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl Sub for Vec2Board {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Consumption {
    energy: Energy,
    materials: Materials,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tower {
    tower_type: TowerType,
    shot_con: Consumption,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TowerType {
    // Damages enemies, needs energy
    LaserShot,

    // Slows enemies down, needs energy
    Microwave,

    // Damages enemies, needs energy and material
    Rocket,

    // Damages enemies, needs energy and material
    Grenade,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Building {
    building_type: BuildingType,
}

impl Building {
    fn new(building_type: BuildingType) -> Self {
        Self { building_type }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum BuildingType {
    Factory,
    PowerPlant,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all_of<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

use std::f32::consts::PI;
use std::fs::{read_dir, read_to_string, DirEntry, File};
use std::io::{BufRead, BufReader, Error, Write};
use std::ops::{Add, Mul, Sub};
use std::path::Path;

pub fn save_board_to_file(name: &str, board: &Board) -> Result<(), Error> {
    let file_content = serde_json::to_string(board)?;
    let path_str = format!("./maps/{}_map.json", name);
    let path = Path::new(&path_str);
    let mut output = File::create(path)?;
    write!(output, "{}", file_content)?;
    Ok(())
}

pub fn get_all_boards_in_folder() -> Result<Vec<Board>, Error> {
    let mut boards = Vec::<Board>::new();
    for dir_entry in read_dir("./maps/")? {
        let input = read_to_string(dir_entry?.path())?;
        boards.push(serde_json::from_str(&input)?);
    }
    Ok(boards)
}

pub fn add_row(label: &str, widget: impl bevy_egui::egui::Widget, ui: &mut bevy_egui::egui::Ui) {
    let width_right_col = ui.available_width() - 200.0;
    ui.horizontal(|ui| {
        ui.set_style(bevy_egui::egui::Style {
            spacing: bevy_egui::egui::style::Spacing {
                slider_width: width_right_col - 60.0,
                ..Default::default()
            },
            ..Default::default()
        });
        ui.add_sized([200., 60.], bevy_egui::egui::Label::new(label));
        ui.add_sized([width_right_col, 60.0], widget);
    });
}

pub fn add_ok_cancel_row(ui: &mut bevy_egui::egui::Ui) -> (bool, bool) {
    let mut is_clicked = (false, false);
    ui.horizontal(|ui| {
        if ui
            .add_sized([200., 60.], bevy_egui::egui::widgets::Button::new("Cancel"))
            .clicked()
        {
            is_clicked.1 = true;
        }
        if ui
            .add_sized([200., 60.], bevy_egui::egui::widgets::Button::new("OK"))
            .clicked()
        {
            is_clicked.0 = true;
        }
    });
    is_clicked
}

pub fn add_error_box(err_text: &str, ui: &mut bevy_egui::egui::Ui) {
    bevy_egui::egui::Frame::none()
        .fill(bevy_egui::egui::Color32::LIGHT_RED)
        .stroke(bevy_egui::egui::Stroke::new(
            2.,
            bevy_egui::egui::Color32::DARK_RED,
        ))
        .inner_margin(3.)
        .outer_margin(2.)
        .show(ui, |ui| ui.add(bevy_egui::egui::Label::new(err_text)));
}

pub fn add_popup_window<R>(
    egui_ctx: &mut ResMut<bevy_egui::EguiContext>,
    title: &str,
    content: impl FnOnce(&mut bevy_egui::egui::Ui) -> R,
) {
    bevy_egui::egui::Window::new(title)
        .fixed_size((400., 200.))
        .collapsible(false)
        .anchor(bevy_egui::egui::Align2::CENTER_CENTER, (0., 0.))
        .show(egui_ctx.ctx_mut(), |ui| {
            // Content
            ui.add_space(10.);
            content(ui);
        });
}

pub fn clone_opt_ref<T: Clone>(opt: Option<&T>) -> Option<T> {
    match opt {
        Some(t) => Some(t.clone()),
        None => None,
    }
}

pub struct TileResizeParams {
    pub tile_size: f32,
    pub tile_size_vec: Vec2,
    pub tile_inner_size: Vec2,
    pub board_start_x: f32,
    pub board_start_y: f32,
    pub board_size: Vec2,
}

impl TileResizeParams {
    pub fn new(board: &Board, start: Vec2, end: Vec2) -> Self {
        let board_width_px = end.x - start.x;
        let board_height_px = end.y - start.y;

        // the tiles are quadratic, so use the smaller size
        let tile_size = get_tile_size_px(board_width_px, board_height_px, board);
        let tile_inner_size = Vec2::new(tile_size - 10., tile_size - 10.);

        Self {
            tile_size,
            tile_size_vec: Vec2::new(tile_size, tile_size),
            tile_inner_size,

            // Think from the middle of the sceen
            board_start_x: (start.x - board_width_px) / 2.,
            board_start_y: (board_height_px - start.y) / 2.,
            board_size: Vec2::new(
                tile_size * board.width as f32,
                tile_size * board.height as f32,
            ),
        }
    }

    pub fn from_start_to_win_end(window: &Window, board: &Board, start: Vec2) -> Self {
        Self::new(board, start, Vec2::new(window.width(), window.height()))
    }
}

pub fn get_tile_size_px(available_width_px: f32, available_height_px: f32, board: &Board) -> f32 {
    let tile_width_px = available_width_px / board.width as f32;
    let tile_height_px = available_height_px / board.height as f32;

    if tile_height_px > tile_width_px {
        tile_width_px
    } else {
        tile_height_px
    }
}

pub fn get_tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::TowerGround(_) => Color::GOLD,
        Tile::BuildingGround(_) => Color::ANTIQUE_WHITE,
        Tile::Road => Color::GRAY,
        Tile::Empty => Color::DARK_GRAY,
    }
}

pub fn is_hover(cursor_pos: Vec2, sprite: &Sprite, transform: &Transform) -> bool {
    if let Some(size) = sprite.custom_size {
        cursor_pos.x >= transform.translation.x
            && cursor_pos.x <= transform.translation.x + size.x
            && cursor_pos.y >= transform.translation.y - size.y
            && cursor_pos.y <= transform.translation.y
    } else {
        false
    }
}

pub fn road_end_shape(tile_size: f32, translation: Vec3) -> ShapeBundle {
    let shape = shapes::RegularPolygon {
        sides: 8,
        feature: shapes::RegularPolygonFeature::Radius(tile_size / 3.),
        ..shapes::RegularPolygon::default()
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SEA_GREEN),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 8.),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}
pub fn enemy_normal_shape(tile_size: f32, translation: Vec3) -> ShapeBundle {
    let shape = shapes::RegularPolygon {
        sides: 5,
        feature: shapes::RegularPolygonFeature::Radius(tile_size / 8.),
        ..shapes::RegularPolygon::default()
    };

    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::MAROON),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, tile_size / 16.),
        },
        Transform {
            translation,
            ..Default::default()
        },
    )
}
