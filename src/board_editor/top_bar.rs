use super::{
    actions::EditorActionEvent,
    popups::{EditBoardWindow, LoadBoardWindow, NewBoardWindow, Popups, SaveBoardWindow},
    BoardEditor, LEFT_BAR_WIDTH_PX, TOP_BAR_HEIGHT_PX,
};
use crate::{board::Board, utils::add_error_box};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Response, TopBottomPanel, Ui},
    EguiContexts,
};

pub(super) fn add_top_menu_bar(
    mut egui_ctx: EguiContexts,
    mut popup: ResMut<Popups>,
    mut actions: EventWriter<EditorActionEvent>,
    state: Res<BoardEditor>,
    board: Res<Board>,
) {
    TopBottomPanel::top("map_editor_top_bar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(super::TOP_BAR_HEIGHT_PX);
        ui.horizontal(|ui| {
            if add_top_bar_button("Back", ui).clicked() {
                actions.send(EditorActionEvent::Leave);
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
                    _ => Popups::Edit(EditBoardWindow::new(&board)),
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
