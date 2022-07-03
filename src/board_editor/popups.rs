use super::actions::EditorActionEvent;
use crate::{
    board::Board,
    utils::{add_error_box, add_row, get_all_boards_in_folder},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, TextEdit},
    EguiContext,
};

pub(super) enum Popups {
    Load(LoadBoardWindow),
    Save(SaveBoardWindow),
    New(NewBoardWindow),
    Edit(EditBoardWindow),
    None,
}

impl Popups {
    pub fn is_open(&self) -> bool {
        !matches!(self, Popups::None)
    }
}

pub(super) struct NewBoardWindow {
    width: u8,
    height: u8,
}

impl Default for NewBoardWindow {
    fn default() -> Self {
        Self {
            width: 10,
            height: 6,
        }
    }
}

pub(super) struct EditBoardWindow {
    width: u8,
    height: u8,
}

impl EditBoardWindow {
    pub fn new(board: &Board) -> Self {
        Self {
            width: board.width,
            height: board.height,
        }
    }
}

pub(super) struct LoadBoardWindow {
    boards: Vec<Board>,
    err_text: Option<String>,
}

impl Default for LoadBoardWindow {
    fn default() -> Self {
        match get_all_boards_in_folder() {
            Ok(boards) => Self {
                boards,
                err_text: None,
            },
            Err(err) => Self {
                boards: Vec::new(),
                err_text: Some(err.to_string()),
            },
        }
    }
}

#[derive(Default)]
pub(super) struct SaveBoardWindow {
    pub map_file_name: String,
    pub err_text: Option<String>,
}

pub(super) fn add_save_board_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut popup: ResMut<Popups>,
    mut actions: EventWriter<EditorActionEvent>,
) {
    let mut is_close = false;
    if let Popups::Save(popup) = &mut *popup {
        add_popup_window(&mut egui_ctx, "Save map", |ui| {
            add_row(
                "Map name",
                TextEdit::singleline(&mut popup.map_file_name).margin(egui::Vec2::new(10., 16.)),
                ui,
            );
            ui.add_space(10.);

            // Ok/Cancel Buttons
            ui.add_space(10.);
            let (is_ok, is_cancel) = add_ok_cancel_row(ui);

            if is_ok {
                actions.send(EditorActionEvent::Save);
            } else if is_cancel {
                is_close = true;
            }

            // Error container
            if let Some(err_text) = &popup.err_text {
                add_error_box(err_text, ui);
            }
        });
    }
    if is_close {
        *popup = Popups::None;
    }
}

pub(super) fn add_load_board_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut popup: ResMut<Popups>,
    actions: EventWriter<EditorActionEvent>,
) {
    let mut is_close = false;
    if let Popups::Load(popup) = &mut *popup {
        add_popup_window(&mut egui_ctx, "Load map", |ui| {
            add_load_board_select(ui, popup, actions);
            ui.add_space(10.);
            if ui
                .add_sized([400., 60.], bevy_egui::egui::widgets::Button::new("Cancel"))
                .clicked()
            {
                is_close = true;
            }
            if let Some(err_text) = &popup.err_text {
                add_error_box(err_text, ui);
            }
        });
    }
    if is_close {
        *popup = Popups::None;
    }
}

fn add_load_board_select(
    ui: &mut egui::Ui,
    load_win: &mut LoadBoardWindow,
    mut actions: EventWriter<EditorActionEvent>,
) {
    egui::containers::ScrollArea::vertical().show(ui, |ui| {
        for board in &load_win.boards {
            if ui
                .add_sized(
                    [400., 60.],
                    bevy_egui::egui::widgets::Button::new(&board.name),
                )
                .clicked()
            {
                actions.send(EditorActionEvent::Load(board.clone()));
                break;
            }
        }
    });
}

pub(super) fn add_new_board_window(
    egui_ctx: ResMut<EguiContext>,
    mut popup: ResMut<Popups>,
    mut actions: EventWriter<EditorActionEvent>,
) {
    let mut is_close = false;
    if let Popups::New(popup) = &mut *popup {
        let (is_ok, is_cancel) =
            add_new_edit_popup(egui_ctx, &mut popup.width, &mut popup.height, "New map");
        if is_ok {
            actions.send(EditorActionEvent::New((popup.width, popup.height)));
        } else if is_cancel {
            is_close = true;
        }
    }
    if is_close {
        *popup = Popups::None;
    }
}

pub(super) fn add_edit_board_window(
    egui_ctx: ResMut<EguiContext>,
    mut popup: ResMut<Popups>,
    mut actions: EventWriter<EditorActionEvent>,
) {
    let mut is_close = false;
    if let Popups::Edit(popup) = &mut *popup {
        let (is_ok, is_cancel) =
            add_new_edit_popup(egui_ctx, &mut popup.width, &mut popup.height, "Edit size");
        if is_ok {
            actions.send(EditorActionEvent::Edit((popup.width, popup.height)));
        } else if is_cancel {
            is_close = true;
        }
    }
    if is_close {
        *popup = Popups::None;
    }
}

pub(super) fn add_new_edit_popup(
    mut egui_ctx: ResMut<EguiContext>,
    width: &mut u8,
    height: &mut u8,
    title: &str,
) -> (bool, bool) {
    let (mut is_ok, mut is_cancel) = (false, false);
    add_popup_window(&mut egui_ctx, title, |ui| {
        // Width
        let width_silder = egui::Slider::new(width, 3..=32)
            .show_value(true)
            .clamp_to_range(true);
        add_row("Width", width_silder, ui);

        // Height
        let height_silder = egui::Slider::new(height, 3..=32)
            .show_value(true)
            .clamp_to_range(true);
        add_row("Height", height_silder, ui);

        // Ok/Cancel Buttons
        ui.add_space(10.);
        (is_ok, is_cancel) = add_ok_cancel_row(ui);
    });
    (is_ok, is_cancel)
}

fn add_ok_cancel_row(ui: &mut bevy_egui::egui::Ui) -> (bool, bool) {
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

fn add_popup_window<R>(
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
