use crate::{
    game::{
        actions::GameActionMessage,
        actions::GameActionMessage::{BackToMainMenu, Continue},
    },
    menu_panels::{controls_content, settings_content},
    user::Settings,
    TITLE,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, Align, Button, CentralPanel, Color32, Frame, Label, Layout, RichText, ScrollArea,
        Stroke,
    },
    EguiContexts,
};

const PAUSE_MENU_WIDTH: f32 = 360.;
const PAUSE_PANEL_WIDTH: f32 = 720.;
const PAUSE_BUTTON_HEIGHT: f32 = 58.;

#[derive(Resource, Default)]
pub(super) struct PauseMenu {
    screen: PauseMenuScreen,
}

#[derive(Default)]
enum PauseMenuScreen {
    #[default]
    Main,
    Settings,
    Controls,
}

pub(super) fn reset_pause_menu(mut pause_menu: ResMut<PauseMenu>) {
    pause_menu.screen = PauseMenuScreen::Main;
}

pub(super) fn pause_menu(
    mut egui_ctx: EguiContexts,
    mut pause_menu: ResMut<PauseMenu>,
    mut settings: ResMut<Settings>,
    mut actions: MessageWriter<GameActionMessage>,
) {
    let Ok(ctx) = egui_ctx.ctx_mut() else { return };
    CentralPanel::default()
        .frame(Frame::new().fill(Color32::from_rgba_premultiplied(0, 0, 0, 128)))
        .show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space((ui.available_height() * 0.14).max(32.));
                match pause_menu.screen {
                    PauseMenuScreen::Main => pause_card(ui, &mut pause_menu, &mut actions),
                    PauseMenuScreen::Settings => settings_card(ui, &mut pause_menu, &mut settings),
                    PauseMenuScreen::Controls => controls_card(ui, &mut pause_menu),
                }
            });
        });
}

fn menu_frame() -> Frame {
    Frame::new()
        .fill(Color32::from_rgba_premultiplied(12, 12, 14, 210))
        .stroke(Stroke::new(
            1.,
            Color32::from_rgba_premultiplied(255, 255, 255, 45),
        ))
        .corner_radius(14.)
        .inner_margin(egui::Margin::symmetric(28, 24))
}

fn pause_card(
    ui: &mut egui::Ui,
    pause_menu: &mut PauseMenu,
    actions: &mut MessageWriter<GameActionMessage>,
) {
    menu_frame().show(ui, |ui| {
        ui.set_width(PAUSE_MENU_WIDTH);
        ui.vertical_centered(|ui| {
            ui.add(Label::new(RichText::new(TITLE).heading()));
            ui.add_space(6.);
            ui.label(RichText::new("Paused").color(Color32::from_gray(190)));
            ui.add_space(24.);

            if pause_button(ui, "Continue").clicked() {
                actions.write(Continue);
            }

            if pause_button(ui, "Settings").clicked() {
                pause_menu.screen = PauseMenuScreen::Settings;
            }

            if pause_button(ui, "Controls").clicked() {
                pause_menu.screen = PauseMenuScreen::Controls;
            }

            if pause_button(ui, "Back to main").clicked() {
                actions.write(BackToMainMenu);
            }
        });
    });
}

fn pause_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add_sized(
        [PAUSE_MENU_WIDTH, PAUSE_BUTTON_HEIGHT],
        Button::new(RichText::new(text).strong()),
    )
}

fn settings_card(ui: &mut egui::Ui, pause_menu: &mut PauseMenu, settings: &mut Settings) {
    menu_frame().show(ui, |ui| {
        ui.set_width(PAUSE_PANEL_WIDTH);
        panel_header(ui, "Settings", pause_menu);
        ui.add_space(16.);
        settings_content(ui, settings);
    });
}

fn controls_card(ui: &mut egui::Ui, pause_menu: &mut PauseMenu) {
    menu_frame().show(ui, |ui| {
        ui.set_width(PAUSE_PANEL_WIDTH);
        panel_header(ui, "Controls", pause_menu);
        ui.add_space(16.);

        ScrollArea::vertical()
            .max_height((ui.available_height() - 20.).max(220.))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                controls_content(ui, false);
            });
    });
}

fn panel_header(ui: &mut egui::Ui, title: &str, pause_menu: &mut PauseMenu) {
    ui.horizontal(|ui| {
        if ui.button("Back").clicked() {
            pause_menu.screen = PauseMenuScreen::Main;
        }
        ui.add_space(12.);
        ui.add(Label::new(RichText::new(title).heading()));
    });
}
