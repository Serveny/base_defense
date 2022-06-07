pub struct MenuPlugin;
use crate::{
    board::ActionBoard,
    utils::{add_error_box, add_row, get_all_boards_in_folder, GameState},
    TITLE,
};
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, CentralPanel, Color32, Frame, Label, Response, RichText, ScrollArea, SidePanel},
    EguiContext,
};

const SIDE_BAR_WIDTH: f32 = 300.0;

// State used for the current menu screen
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    NewGame,
    Settings,
    Disabled,
}

struct NewGameMenu {
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

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(startup_menu))
            .add_system_set(
                SystemSet::on_enter(MenuState::NewGame).with_system(new_game_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(MenuState::NewGame)
                    .with_system(add_new_game_menu.after(startup_menu)),
            )
            .add_state(MenuState::Disabled);
    }
}

fn startup_menu(
    mut game_state: ResMut<State<GameState>>,
    mut menu_state: ResMut<State<MenuState>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut app_exit_events: EventWriter<AppExit>,
    settings: ResMut<crate::user::Settings>,
) {
    add_main_menu(
        &mut game_state,
        &mut menu_state,
        &mut egui_ctx,
        &mut app_exit_events,
    );

    match *menu_state.current() {
        MenuState::Settings => add_settings(&mut egui_ctx, settings),
        MenuState::Disabled => menu_state.set(MenuState::Main).unwrap(),
        _ => (),
    }
}

fn add_main_menu(
    game_state: &mut ResMut<State<GameState>>,
    menu_state: &mut ResMut<State<MenuState>>,
    egui_ctx: &mut ResMut<EguiContext>,
    app_exit_events: &mut EventWriter<AppExit>,
) {
    SidePanel::left("left_panel")
        .resizable(false)
        .default_width(SIDE_BAR_WIDTH)
        .frame(Frame::none().fill(Color32::from_rgba_premultiplied(0, 0, 0, 50)))
        .show(egui_ctx.ctx_mut(), |ui| {
            // Title
            ui.add_sized(
                [SIDE_BAR_WIDTH, 60.0],
                Label::new(RichText::new(TITLE).heading()),
            );

            if add_menu_button("Play", ui).clicked() {
                menu_state.set(MenuState::NewGame).unwrap_or_else(|_| {
                    menu_state.set(MenuState::Main).unwrap();
                });
            }

            if add_menu_button("Map Editor", ui).clicked() {
                leave_menu(GameState::MapEditor, menu_state, game_state);
            }

            if add_menu_button("Settings", ui).clicked() {
                menu_state.set(MenuState::Settings).unwrap_or_else(|_| {
                    menu_state.set(MenuState::Main).unwrap();
                });
            }

            if add_menu_button("Quit", ui).clicked() {
                app_exit_events.send(AppExit);
            }
        });
}

fn add_menu_button(text: &str, ui: &mut egui::Ui) -> Response {
    ui.add_sized([SIDE_BAR_WIDTH, 60.0], egui::Button::new(text).frame(false))
}

fn add_settings(egui_ctx: &mut ResMut<EguiContext>, mut settings: ResMut<crate::user::Settings>) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(ui.available_height());
        ScrollArea::vertical().show(ui, |ui| {
            let volume_silder = egui::Slider::new(&mut settings.volume.0, 0..=100)
                .show_value(false)
                .clamp_to_range(true);
            add_row("Volume", volume_silder, ui);
        });
    });
}

fn new_game_menu_setup(mut commands: Commands) {
    commands.init_resource::<NewGameMenu>();
}

fn add_new_game_menu(
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
                        ui.set_height(60.);
                        ui.set_width(ui.available_width());
                        let boards = &new_game_menu.boards;
                        let mut selected_i = new_game_menu.selected_board_index;

                        for (i, board) in boards.iter().enumerate() {
                            ui.selectable_value(&mut selected_i, i, board.name());
                        }
                        new_game_menu.selected_board_index = selected_i;
                    });
            });
        });
        // ui.group(|ui| {
        //     ui.set_max_height(400.);
        //     ui.set_width(390.);
        //     ui.label("Select map");
        //     egui::containers::ScrollArea::vertical().show(ui, |ui| {
        //         let boards = &new_game_menu.boards;
        //         let mut selected_i = new_game_menu.selected_board_index;

        //         for (i, board) in boards.iter().enumerate() {
        //             ui.selectable_value(&mut selected_i, i, &board.name);
        //         }
        //         new_game_menu.selected_board_index = selected_i;
        //     });
        // });
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

fn leave_menu(
    to: GameState,
    menu_state: &mut ResMut<State<MenuState>>,
    game_state: &mut ResMut<State<GameState>>,
) {
    menu_state.set(MenuState::Disabled).unwrap();
    game_state.set(to).unwrap();
}
