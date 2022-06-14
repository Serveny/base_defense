use crate::{
    board::{draw_board, ActionBoard},
    game::Game,
    utils::{add_error_box, get_all_boards_in_folder, Difficulty, GameState},
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
    difficulty: Difficulty,
    err_text: Option<String>,
}

impl NewGameMenu {
    fn get_selected_board(&self) -> &ActionBoard {
        &self.boards[self.selected_board_index]
    }
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
                difficulty: Difficulty::Easy,
                err_text: None,
            },
            Err(err) => Self {
                boards: Vec::new(),
                selected_board_index: 0,
                difficulty: Difficulty::Easy,
                err_text: Some(err.to_string()),
            },
        }
    }
}

pub(super) fn new_game_menu_setup(mut commands: Commands) {
    let resource = NewGameMenu::default();
    draw_board(
        &mut commands,
        resource.get_selected_board(),
        UVec2::new(400, 400),
        Vec2::new(400., 400.),
    );
    commands.insert_resource(resource);
}

pub(super) fn add_new_game_menu(
    mut cmds: Commands,
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
            ui.horizontal(|ui| {
                ui.add_sized([200., 60.], bevy_egui::egui::Label::new("Difficulty"));
                enum_as_radio_select(ui, &mut new_game_menu.difficulty);
            })
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
                        cmds.insert_resource(Game::new(new_game_menu.get_selected_board().clone()));
                        leave_menu(GameState::Game, &mut menu_state, &mut game_state);
                    }
                });
            });
    });
}
pub fn enum_as_radio_select<T: std::fmt::Display + strum::IntoEnumIterator + PartialEq>(
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
