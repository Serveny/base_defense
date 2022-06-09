use crate::{
    board::ActionBoard,
    utils::{add_error_box, get_all_boards_in_folder, GameState},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CentralPanel},
    EguiContext,
};

use super::{leave_menu, MenuState};

pub(super) struct NewGameMenu {
    boards: Vec<ActionBoard>,
    selected_board_index: usize,
    err_text: Option<String>,
}

impl Default for NewGameMenu {
    fn default() -> Self {
        match get_all_boards_in_folder() {
            Ok(boards) => Self {
                boards: boards
                    .into_iter()
                    .map(|board| ActionBoard::new(board))
                    .collect(),
                selected_board_index: 0,
                err_text: None,
            },
            Err(err) => Self {
                boards: Vec::new(),
                selected_board_index: 0,
                err_text: Some(err.to_string()),
            },
        }
    }
}

pub(super) fn new_game_menu_setup(mut commands: Commands) {
    commands.init_resource::<NewGameMenu>();
}

pub(super) fn add_new_game_menu(
    mut egui_ctx: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    mut menu_state: ResMut<State<MenuState>>,
    mut new_game_menu: ResMut<NewGameMenu>,
) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(ui.available_height());
        ui.vertical_centered(|ui| {
            ui.heading("New Game");

            // Error container
            if let Some(err_text) = &new_game_menu.err_text {
                add_error_box(err_text, ui);
            }
            ui.horizontal(|ui| {
                ui.add_sized([200., 60.], bevy_egui::egui::Label::new("Map"));
                let selected = new_game_menu.boards[new_game_menu.selected_board_index].name();
                egui::containers::ComboBox::from_label("")
                    .selected_text(selected)
                    .show_ui(ui, |ui| {
                        ui.set_width(400.);
                        let boards = &new_game_menu.boards;
                        let mut selected_i = new_game_menu.selected_board_index;

                        for (i, board) in boards.iter().enumerate() {
                            ui.selectable_value(&mut selected_i, i, board.name());
                        }
                        new_game_menu.selected_board_index = selected_i;
                    });
            });
        });
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
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized([400., 60.], bevy_egui::egui::widgets::Button::new("Play"))
                        .clicked()
                    {
                        leave_menu(GameState::Game, &mut menu_state, &mut game_state);
                    }
                });
            });
    });
}
