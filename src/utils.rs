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
#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
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

use crate::assets::FONT_QUICKSAND;
use bevy::render::camera::ScalingMode;
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
    Angle::radians((target - pos).angle_to(Vec2::new(0., 1.)))
}

pub fn pos_to_quat(pos: Vec2Board, target: Vec2Board) -> Quat {
    Quat::from_rotation_z(-pos_to_angle(pos, target).radians)
}

pub fn zoom_cam_to_board(board: &Board, q_cam: &mut CamMutQuery, win: &Window) {
    let aspect_ratio_margin = cam_margin(board, win);
    let height = (board.height as f32 + aspect_ratio_margin.y) * TILE_SIZE;
    let width = (board.width as f32 + aspect_ratio_margin.x) * TILE_SIZE;
    let mut projection = q_cam.single_mut();
    projection.scaling_mode = ScalingMode::Fixed { width, height };
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

pub fn cursor_pos(wnd: &Window, q_cam: CamQuery) -> Option<Vec2Board> {
    let (camera, camera_transform) = q_cam.single();

    if let Some(screen_pos) = wnd
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
    {
        return Some((screen_pos / TILE_SIZE).into());
    }
    None
}

pub fn text_bundle(
    text: &str,
    color: Color,
    assets: &AssetServer,
    left: Val,
    bottom: Val,
) -> impl Bundle {
    (
        Node {
            border: UiRect::all(Val::Px(2.)),
            position_type: PositionType::Absolute,
            left: left / TILE_SIZE,
            bottom: bottom / TILE_SIZE,
            ..default()
        },
        BorderColor(Color::srgba(1., 1., 1., 0.05)),
        Text(text.to_string()),
        TextFont {
            font: assets.load(FONT_QUICKSAND),
            font_size: 30.,
            ..default()
        },
        TextColor(color),
        TextLayout {
            justify: JustifyText::Center,
            linebreak: LineBreak::NoWrap,
        },
        BackgroundColor(Color::srgba(1., 1., 1., 0.05)),
    )
}

pub fn visible(is_visible: bool) -> Visibility {
    match is_visible {
        true => Visibility::Inherited,
        false => Visibility::Hidden,
    }
}
