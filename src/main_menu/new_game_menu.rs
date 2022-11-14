use super::actions::MenuActionEvent;
use crate::{
    board::{Board, BoardCache},
    game::Game,
    utils::{add_error_box, get_all_boards_in_folder, Difficulty},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CentralPanel},
    EguiContext,
};
use std::error::Error;

#[derive(Resource)]
pub(super) struct NewGameMenu {
    boards: Vec<(Board, BoardCache)>,
    selected_board_index: usize,
    difficulty: Difficulty,
    err_text: Option<String>,
}

impl NewGameMenu {
    fn selected_board(&self) -> Option<&(Board, BoardCache)> {
        self.boards.get(self.selected_board_index)
    }

    fn new(boards: Vec<Board>) -> Self {
        Self {
            boards: boards
                .into_iter()
                .filter_map(|board| {
                    let cache = BoardCache::new(&board);
                    if cache.validate().is_ok() {
                        return Some((board, cache));
                    }
                    None
                })
                .collect(),
            selected_board_index: 0,
            difficulty: Difficulty::Easy,
            err_text: None,
        }
    }

    fn new_error(err: Box<dyn Error>) -> Self {
        Self {
            boards: Vec::new(),
            selected_board_index: 0,
            difficulty: Difficulty::Easy,
            err_text: Some(err.to_string()),
        }
    }
}

impl Default for NewGameMenu {
    fn default() -> Self {
        match get_all_boards_in_folder() {
            Ok(boards) => Self::new(boards),
            Err(err) => Self::new_error(err),
        }
    }
}

pub(super) fn new_game_menu_setup(mut commands: Commands) {
    commands.init_resource::<NewGameMenu>();
}

pub(super) fn add_new_game_menu(
    mut egui_ctx: ResMut<EguiContext>,
    mut new_game_menu: ResMut<NewGameMenu>,
    actions: EventWriter<MenuActionEvent>,
) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(ui.available_height());
        ui.vertical_centered(|ui| {
            ui.heading("New Game");

            // Error container
            if let Some(err_text) = &new_game_menu.err_text {
                add_error_box(err_text, ui);
            }
            board_select(ui, &mut new_game_menu);
            difficulty_select(ui, &mut new_game_menu);
        });
        bottom_panel(ui, &mut new_game_menu, actions);
    });
}

fn board_select(ui: &mut egui::Ui, new_game_menu: &mut NewGameMenu) {
    ui.horizontal(|ui| {
        ui.add_sized([200., 60.], bevy_egui::egui::Label::new("Map"));
        if let Some(selected) = new_game_menu.selected_board() {
            egui::containers::ComboBox::from_label("")
                .selected_text(&selected.0.name)
                .show_ui(ui, |ui| {
                    ui.set_width(400.);
                    let boards = &new_game_menu.boards;
                    let mut selected_i = new_game_menu.selected_board_index;

                    for (i, board) in boards.iter().enumerate() {
                        ui.selectable_value(&mut selected_i, i, &board.0.name);
                    }
                    new_game_menu.selected_board_index = selected_i;
                });
        } else {
            ui.add_sized(
                [200., 60.],
                bevy_egui::egui::Label::new("No valid maps found."),
            );
        }
    });
}

fn difficulty_select(ui: &mut egui::Ui, new_game_menu: &mut NewGameMenu) {
    ui.horizontal(|ui| {
        ui.add_sized([200., 60.], bevy_egui::egui::Label::new("Difficulty"));
        enum_as_radio_select(ui, &mut new_game_menu.difficulty);
    });
}

fn bottom_panel(
    ui: &mut egui::Ui,
    new_game_menu: &mut NewGameMenu,
    actions: EventWriter<MenuActionEvent>,
) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .default_height(60.)
        .frame(egui::Frame {
            stroke: egui::Stroke {
                width: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| play_button(ui, new_game_menu, actions));
        });
}

fn play_button(
    ui: &mut egui::Ui,
    new_game_menu: &mut NewGameMenu,
    mut actions: EventWriter<MenuActionEvent>,
) {
    if ui
        .add_sized([400., 60.], bevy_egui::egui::widgets::Button::new("Play"))
        .clicked()
    {
        let (board, board_cache) = new_game_menu
            .boards
            .get(new_game_menu.selected_board_index)
            .unwrap()
            .clone();
        actions.send(MenuActionEvent::StartNewGame(
            Game::new(new_game_menu.difficulty),
            board,
            board_cache,
        ));
    }
}

fn enum_as_radio_select<T: std::fmt::Display + strum::IntoEnumIterator + PartialEq>(
    ui: &mut bevy_egui::egui::Ui,
    selected: &mut T,
) {
    for enum_value in T::iter() {
        let is_selected = enum_value == *selected;
        let text = enum_value.to_string();
        if add_selectable_label(ui, is_selected, &text) {
            *selected = enum_value;
        }
    }
}

fn add_selectable_label(ui: &mut bevy_egui::egui::Ui, is_selected: bool, text: &str) -> bool {
    ui.add_sized(
        [200., 60.],
        bevy_egui::egui::widgets::SelectableLabel::new(is_selected, text),
    )
    .clicked()
}
