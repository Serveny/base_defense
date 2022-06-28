// #![allow(unused)]
use crate::board::step::BoardDirection;
use crate::board::Board;
use bevy::prelude::*;
use euclid::Angle;
use serde::{Deserialize, Serialize};

pub mod towers;

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

#[derive(Default, Deref, DerefMut, Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct Vec2Board(Vec2);

impl Vec2Board {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn from_uvec2_middle(uvec2: &UVec2) -> Self {
        Self::new(uvec2.x as f32 + 0.5, uvec2.y as f32 + 0.5)
    }

    // Only for not diagonal vec2
    pub fn distance_from_zero(&self) -> f32 {
        self.x.abs() + self.y.abs()
    }

    pub fn distance(&self, other: Vec2Board) -> f32 {
        ((self.x - other.x).powi(2) - (self.y - other.y).powi(2))
            .sqrt()
            .abs()
    }

    pub fn add_in_direction(&mut self, distance: f32, direction: BoardDirection) {
        match direction {
            BoardDirection::Up => self.0.y -= distance,
            BoardDirection::Right => self.0.x += distance,
            BoardDirection::Down => self.0.y += distance,
            BoardDirection::Left => self.0.x -= distance,
        };
    }

    pub fn degre_between_y(&self, other: Vec2Board) -> Angle<f32> {
        let b = self.distance(other);
        let c = (self.y - other.y).abs();
        Angle::degrees((b * c).acos())
    }

    pub fn to_vec3(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }
}

impl From<Vec2> for Vec2Board {
    fn from(vec2: Vec2) -> Self {
        Self(vec2)
    }
}

impl From<Vec2Board> for Vec2 {
    fn from(vec2: Vec2Board) -> Self {
        Self::new(vec2.x, vec2.y)
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

impl AddAssign for Vec2Board {
    fn add_assign(&mut self, other: Vec2Board) {
        self.x += other.x;
        self.y += other.y;
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Building {
    building_type: BuildingType,
}

#[allow(dead_code)]
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

use std::error::Error;
use std::fs::{read_dir, read_to_string, DirEntry, File};
use std::io::Write;
use std::ops::{Add, AddAssign, Mul, Sub};
use std::path::Path;

pub fn save_board_to_file(name: &str, board: &Board) -> Result<(), Box<dyn Error>> {
    let mut output = File::create(Path::new(&format!("./maps/{}_map.json", name)))?;
    write!(output, "{}", serde_json::to_string(board)?)?;
    Ok(())
}

pub fn get_all_boards_in_folder() -> Result<Vec<Board>, Box<dyn Error>> {
    let mut boards = Vec::<Board>::new();
    for dir_entry in read_dir("./maps/")? {
        boards.push(read_map(dir_entry)?);
    }
    Ok(boards)
}

fn read_map(dir_entry: Result<DirEntry, std::io::Error>) -> Result<Board, Box<dyn Error>> {
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
