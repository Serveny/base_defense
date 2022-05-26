use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Response, TopBottomPanel, Ui},
    EguiContext,
};

use crate::utils::{add_error_box, GameState};

use super::{
    popups::{EditBoardWindow, LoadBoardWindow, NewBoardWindow, Popups, SaveBoardWindow},
    BoardEditorState, LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX,
};

pub(super) fn add_top_menu_bar(
    mut egui_ctx: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    mut popup: ResMut<Popups>,
    state: Res<BoardEditorState>,
) {
    TopBottomPanel::top("map_editor_top_bar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(super::TOP_BAR_HEIGHT_PX);
        ui.horizontal(|ui| {
            if add_top_bar_button("Back", ui).clicked() {
                game_state.set(GameState::Menu).unwrap();
            }

            ui.add_space(super::LEFT_BAR_WIDTH_PX - 80.);

            if add_top_bar_button("Load", ui).clicked() {
                *popup = match *popup {
                    Popups::Load(_) => Popups::None,
                    _ => Popups::Load(LoadBoardWindow::default()),
                }
            }

            if add_top_bar_button("Save", ui).clicked() {
                *popup = match *popup {
                    Popups::Save(_) => Popups::None,
                    _ => Popups::Save(SaveBoardWindow::default()),
                }
            }

            if add_top_bar_button("New", ui).clicked() {
                *popup = match *popup {
                    Popups::New(_) => Popups::None,
                    _ => Popups::New(NewBoardWindow::default()),
                }
            }

            if add_top_bar_button("Edit", ui).clicked() {
                *popup = match *popup {
                    Popups::Edit(_) => Popups::None,
                    _ => Popups::Edit(EditBoardWindow::new(&state.current_map)),
                }
            }

            if let Some(err_text) = &state.err_text {
                ui.add_space(LEFT_BAR_WIDTH_PX - 80.);
                add_error_box(err_text, ui);
            }
        });
    });
}

fn add_top_bar_button(text: &str, ui: &mut Ui) -> Response {
    ui.add_sized(
        [60., TOP_BAR_HEIGHT_PX],
        egui::Button::new(text).frame(false),
    )
}
