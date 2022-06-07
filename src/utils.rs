#![allow(unused)]
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::board::{ActionBoard, Board};

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Menu,
    Game,
    MapEditor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tower {
    tower_type: TowerType,
}

impl Tower {
    fn new(tower_type: TowerType) -> Self {
        Self { tower_type }
    }
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

use std::fs::{read_dir, read_to_string, DirEntry, File};
use std::io::{BufRead, BufReader, Error, Write};
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
