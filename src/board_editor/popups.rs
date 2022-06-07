use super::{repaint, BoardEditorScreen, BoardEditorState};
use crate::{
    board::{ActionBoard, Board},
    utils::{
        add_error_box, add_ok_cancel_row, add_popup_window, add_row, get_all_boards_in_folder,
        save_board_to_file,
    },
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
        match *self {
            Popups::None => false,
            _ => true,
        }
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
    map_file_name: String,
    err_text: Option<String>,
}

pub(super) fn add_save_board_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut editor_state: ResMut<BoardEditorState>,
    mut popup: ResMut<Popups>,
) {
    let mut is_close = false;
    if let Popups::Save(save_win) = &mut *popup {
        add_popup_window(&mut egui_ctx, "Save map", |ui| {
            add_row(
                "Map name",
                TextEdit::singleline(&mut save_win.map_file_name).margin(egui::Vec2::new(10., 16.)),
                ui,
            );
            ui.add_space(10.);

            // Ok/Cancel Buttons
            ui.add_space(10.);
            let (is_ok, is_cancel) = add_ok_cancel_row(ui);

            if is_ok {
                save_win.err_text = None;
                *editor_state.current_map.name_mut() = save_win.map_file_name.clone();
                match save_board_to_file(&save_win.map_file_name, editor_state.current_map.board())
                {
                    Ok(()) => is_close = true,
                    Err(error) => add_error_box(&error.to_string(), ui),
                }
            } else if is_cancel {
                is_close = true;
            }

            // Error container
            if let Some(err_text) = &save_win.err_text {
                add_error_box(err_text, ui);
            }
        });
    }
    if is_close {
        *popup = Popups::None;
    }
}

pub(super) fn add_load_board_window(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut editor_state: ResMut<BoardEditorState>,
    mut popup: ResMut<Popups>,
    query: Query<Entity, With<BoardEditorScreen>>,
    windows: Res<Windows>,
) {
    let mut is_close = false;
    if let Popups::Load(load_win) = &mut *popup {
        add_popup_window(&mut egui_ctx, "Load map", |ui| {
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                for board in &load_win.boards {
                    if ui
                        .add_sized(
                            [400., 60.],
                            bevy_egui::egui::widgets::Button::new(&board.name),
                        )
                        .clicked()
                    {
                        editor_state.current_map = ActionBoard::new(board.clone());
                        repaint(&mut commands, query, &windows, &mut editor_state);
                        editor_state.err_text = match editor_state.current_map.validate() {
                            Ok(_) => None,
                            Err(err) => Some(String::from(err)),
                        };
                        is_close = true;
                        break;
                    }
                }
            });
            ui.add_space(10.);
            if ui
                .add_sized([400., 60.], bevy_egui::egui::widgets::Button::new("Cancel"))
                .clicked()
            {
                is_close = true;
            }
            if let Some(err_text) = &load_win.err_text {
                add_error_box(err_text, ui);
            }
        });
    }
    if is_close {
        *popup = Popups::None;
    }
}

pub(super) fn add_new_board_window(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<Popups>,
    query: Query<Entity, With<BoardEditorScreen>>,
    windows: Res<Windows>,
) {
    let mut is_close = false;
    if let Popups::New(new_win) = &mut *popup {
        let (is_ok, is_cancel) =
            add_new_edit_popup(egui_ctx, &mut new_win.width, &mut new_win.height, "New map");

        if is_ok {
            state.current_map = ActionBoard::empty(new_win.width, new_win.height);
            state.err_text = None;
            repaint(&mut commands, query, &windows, &mut state);
            is_close = true;
        } else if is_cancel {
            is_close = true;
        }
    }
    if is_close {
        *popup = Popups::None;
    }
}

pub(super) fn add_edit_board_window(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<Popups>,
    query: Query<Entity, With<BoardEditorScreen>>,
    windows: Res<Windows>,
) {
    let mut is_close = false;
    if let Popups::Edit(edit_win) = &mut *popup {
        let (is_ok, is_cancel) = add_new_edit_popup(
            egui_ctx,
            &mut edit_win.width,
            &mut edit_win.height,
            "Edit size",
        );

        if is_ok {
            state
                .current_map
                .change_size(edit_win.width, edit_win.height);
            repaint(&mut commands, query, &windows, &mut state);
            is_close = true;
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
