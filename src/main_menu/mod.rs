use self::{
    actions::{menu_actions, MenuActionEvent},
    new_game_menu::{add_new_game_menu, new_game_menu_setup},
};
use crate::{
    utils::{add_row, GameState},
    TITLE,
};
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, CentralPanel, Color32, Frame, Label, Response, RichText, ScrollArea, SidePanel},
    EguiContext,
};

mod actions;
mod new_game_menu;

const SIDE_BAR_WIDTH: f32 = 300.0;

// State used for the current menu screen
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    NewGame,
    Settings,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuActionEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(startup_menu)
                    .with_system(menu_actions),
            )
            .add_system_set(
                SystemSet::on_enter(MenuState::NewGame).with_system(new_game_menu_setup),
            )
            .add_system_set(
                SystemSet::on_update(MenuState::NewGame)
                    .with_system(add_new_game_menu.after(startup_menu)),
            )
            .add_state(MenuState::Main);
    }
}

fn startup_menu(
    mut menu_state: ResMut<State<MenuState>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut app_exit_events: EventWriter<AppExit>,
    actions: EventWriter<MenuActionEvent>,
    settings: ResMut<crate::user::Settings>,
) {
    add_main_menu(
        &mut menu_state,
        &mut egui_ctx,
        &mut app_exit_events,
        actions,
    );

    match *menu_state.current() {
        MenuState::Settings => add_settings(&mut egui_ctx, settings),
        _ => (),
    }
}

fn add_main_menu(
    menu_state: &mut ResMut<State<MenuState>>,
    egui_ctx: &mut ResMut<EguiContext>,
    app_exit_events: &mut EventWriter<AppExit>,
    mut actions: EventWriter<MenuActionEvent>,
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
                actions.send(MenuActionEvent::LeaveMenu(GameState::MapEditor));
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
