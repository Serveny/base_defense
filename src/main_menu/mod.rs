use self::{
    actions::{menu_actions, MenuActionMessage},
    new_game_menu::{add_new_game_menu, new_game_menu_setup},
};
use crate::{
    menu_panels::{controls_content, settings_content},
    utils::GameState,
    TITLE,
};
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, CentralPanel, Color32, Frame, Label, Response, RichText, ScrollArea, SidePanel},
    EguiContexts, EguiPrimaryContextPass,
};

mod actions;
// mod controls;
mod new_game_menu;

const SIDE_BAR_WIDTH: f32 = 300.0;

// State used for the current menu screen
#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
enum MenuState {
    #[default]
    Main,
    NewGame,
    Settings,
    Controls,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MenuActionMessage>()
            .add_systems(
                EguiPrimaryContextPass,
                startup_menu.run_if(in_state(GameState::Menu)),
            )
            .add_systems(Update, menu_actions.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(MenuState::NewGame), new_game_menu_setup)
            .add_systems(
                EguiPrimaryContextPass,
                (add_new_game_menu.after(startup_menu)).run_if(in_state(MenuState::NewGame)),
            )
            .init_state::<MenuState>();
    }
}

fn startup_menu(
    mut set_menu_state: ResMut<NextState<MenuState>>,
    menu_state: Res<State<MenuState>>,
    mut egui_ctx: EguiContexts,
    mut app_exit_events: MessageWriter<AppExit>,
    actions: MessageWriter<MenuActionMessage>,
    settings: ResMut<crate::user::Settings>,
) {
    add_main_menu(
        &menu_state,
        &mut set_menu_state,
        &mut egui_ctx,
        &mut app_exit_events,
        actions,
    );

    if let MenuState::Settings = **menu_state {
        add_settings(&mut egui_ctx, settings)
    }

    if let MenuState::Controls = **menu_state {
        add_controls(&mut egui_ctx)
    }
}

fn add_main_menu(
    menu_state: &State<MenuState>,
    set_menu_state: &mut NextState<MenuState>,
    egui_ctx: &mut EguiContexts,
    app_exit_events: &mut MessageWriter<AppExit>,
    mut actions: MessageWriter<MenuActionMessage>,
) {
    let Ok(ctx) = egui_ctx.ctx_mut() else { return };
    SidePanel::left("left_panel")
        .resizable(false)
        .default_width(SIDE_BAR_WIDTH)
        .frame(Frame::new().fill(Color32::from_rgba_premultiplied(0, 0, 0, 50)))
        .show(ctx, |ui| {
            // Title
            ui.add_sized(
                [SIDE_BAR_WIDTH, 60.0],
                Label::new(RichText::new(TITLE).heading()),
            );

            if add_menu_button("Play", ui).clicked() {
                actions.write(MenuActionMessage::EnterNewGameMenu);
            }

            if add_menu_button("Map Editor", ui).clicked() {
                actions.write(MenuActionMessage::LeaveMenu(GameState::MapEditor));
            }

            if add_menu_button("Controls", ui).clicked() {
                match **menu_state {
                    MenuState::Controls => set_menu_state.set(MenuState::Main),
                    _ => set_menu_state.set(MenuState::Controls),
                }
            }

            if add_menu_button("Settings", ui).clicked() {
                match **menu_state {
                    MenuState::Settings => set_menu_state.set(MenuState::Main),
                    _ => set_menu_state.set(MenuState::Settings),
                }
            }

            if add_menu_button("Quit", ui).clicked() {
                app_exit_events.write(AppExit::Success);
            }
        });
}

fn add_menu_button(text: &str, ui: &mut egui::Ui) -> Response {
    ui.add_sized([SIDE_BAR_WIDTH, 60.0], egui::Button::new(text).frame(false))
}

fn add_settings(egui_ctx: &mut EguiContexts, mut settings: ResMut<crate::user::Settings>) {
    let Ok(ctx) = egui_ctx.ctx_mut() else { return };
    CentralPanel::default().show(ctx, |ui| {
        ui.set_height(ui.available_height());
        ScrollArea::vertical().show(ui, |ui| {
            settings_content(ui, &mut settings);
        });
    });
}

fn add_controls(egui_ctx: &mut EguiContexts) {
    let Ok(ctx) = egui_ctx.ctx_mut() else { return };
    CentralPanel::default().show(ctx, |ui| {
        ui.set_height(ui.available_height());
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                controls_content(ui, true);
            });
    });
}
