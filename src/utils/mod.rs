use crate::assets::StandardAssets;
use crate::board::visualisation::TILE_SIZE;
use crate::board::Board;
use crate::{CamMutQuery, CamQuery};
use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use euclid::Angle;
pub use ingame_time::IngameTime;
pub use ingame_time::IngameTimestamp;
use serde::{Deserialize, Serialize};
pub use vec2_board::Vec2Board;

pub mod buffer;
pub mod buildings;
pub mod collision;
pub mod energy;
pub mod explosions;
pub mod health_bar;
mod ingame_time;
pub mod materials;
pub mod range_circle;
pub mod resource_bar;
pub mod shots;
pub mod speed;
pub mod towers;
mod vec2_board;
pub mod wave;

#[derive(Component, Deref)]
pub struct BoardPos(UVec2);

pub type TilesPerSecond = f32;

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

pub type Energy = f32;
pub type Materials = f32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Amount<T: Default> {
    PerSecond(T),
    Once(T),
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_all_of<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

use std::error::Error;
use std::fs::{read_dir, read_to_string, DirEntry, File};
use std::io::Write;
use std::path::Path;

pub fn save_board_to_file(name: &str, board: &Board) -> Result<(), Box<dyn Error>> {
    let mut output = File::create(Path::new(&format!("./maps/{}_map.json", name)))?;
    write!(output, "{}", serde_json::to_string(board)?)?;
    Ok(())
}

pub fn get_all_boards_in_folder() -> Result<Vec<Board>, Box<dyn Error>> {
    let mut boards = Vec::<Board>::new();
    for dir_entry in read_dir("./maps/")? {
        boards.push(board_from_file(dir_entry)?);
    }
    Ok(boards)
}

fn board_from_file(dir_entry: Result<DirEntry, std::io::Error>) -> Result<Board, Box<dyn Error>> {
    let dir_entry = dir_entry?;
    match serde_json::from_str(&read_to_string(dir_entry.path())?) {
        Ok(board) => Ok(board),
        Err(err) => Err(format!(
            "Invalid json in file '{:?}': {}",
            dir_entry.file_name(),
            err
        )
        .into()),
    }
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
pub fn add_text_row(label: &str, text: &str, ui: &mut bevy_egui::egui::Ui) {
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
        ui.add_sized([200., 60.], bevy_egui::egui::Label::new(text));
    });
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

pub fn pos_to_angle(pos: Vec2Board, target: Vec2Board) -> Angle<f32> {
    Angle::radians((target - pos).angle_between(Vec2::new(0., 1.)))
}

pub fn pos_to_quat(pos: Vec2Board, target: Vec2Board) -> Quat {
    Quat::from_rotation_z(-pos_to_angle(pos, target).radians)
}

pub fn zoom_cam_to_board(board: &Board, mut cam_query: CamMutQuery, windows: &Windows) {
    let win = windows.get_primary().unwrap();
    let margin = cam_margin(board, win);
    let mut cam = cam_query.single_mut();
    (cam.left, cam.right) = (
        -margin.x * TILE_SIZE,
        (board.width as f32 + margin.x) * TILE_SIZE,
    );
    (cam.bottom, cam.top) = (
        -margin.y * TILE_SIZE,
        (board.height as f32 + margin.y) * TILE_SIZE,
    );
}

fn cam_margin(board: &Board, win: &Window) -> Vec2Board {
    let b_w = board.width as f32;
    let b_h = board.height as f32;

    let tile_width_px = win.width() / b_w;
    let tile_height_px = win.height() / b_h;

    if tile_height_px > tile_width_px {
        Vec2Board::new(0., ((win.height() / tile_width_px) - b_h) / 2.)
    } else {
        Vec2Board::new(((win.width() / tile_height_px) - b_w) / 2., 0.)
    }
}

pub fn cursor_pos(wnds: Res<Windows>, q_cam: CamQuery) -> Option<Vec2Board> {
    let (camera, camera_transform) = q_cam.single();
    let wnd = wnds.get_primary().unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        return Some((world_pos.truncate() / TILE_SIZE).into());
    }
    None
}

pub fn text_bundle(
    width: f32,
    text: &str,
    color: Color,
    assets: &StandardAssets,
    transform: Transform,
    horizontal_align: HorizontalAlign,
) -> Text2dBundle {
    Text2dBundle {
        text: Text::from_section(
            text,
            TextStyle {
                font: assets.font.clone(),
                font_size: width / 1.5,
                color,
            },
        )
        .with_alignment(TextAlignment {
            horizontal: horizontal_align,
            vertical: VerticalAlign::Center,
        }),
        transform,
        ..default()
    }
}

pub fn text_background_shape(width: f32, transform: Transform, is_visible: bool) -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::Center,
        extents: Vec2::new(width / 2., width / 10.),
    };
    let mut bundle = GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgba(1., 1., 1., 0.05)),
            outline_mode: StrokeMode::new(Color::rgba(1., 1., 1., 0.05), width / 40.),
        },
        transform,
    );
    bundle.visibility.is_visible = is_visible;
    bundle
}
