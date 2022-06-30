// #![allow(unused)]
use crate::board::Board;
use bevy::prelude::*;
use euclid::Angle;
use serde::{Deserialize, Serialize};
pub use vec2_board::Vec2Board;

pub mod buildings;
pub mod health_bar;
pub mod towers;
mod vec2_board;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Consumption {
    energy: Energy,
    materials: Materials,
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
            err.to_string()
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
    Quat::from_rotation_z(pos_to_angle(pos, target).radians)
}
