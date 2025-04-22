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
    egui::{
        self, CentralPanel, Color32, Frame, Label, Response, RichText, ScrollArea, SidePanel,
        SliderClamping,
    },
    EguiContexts,
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
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuActionEvent>()
            .add_systems(
                Update,
                (startup_menu, menu_actions).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnEnter(MenuState::NewGame), new_game_menu_setup)
            .add_systems(
                Update,
                (add_new_game_menu.after(startup_menu)).run_if(in_state(MenuState::NewGame)),
            )
            .init_state::<MenuState>();
    }
}

fn startup_menu(
    mut set_menu_state: ResMut<NextState<MenuState>>,
    menu_state: Res<State<MenuState>>,
    mut egui_ctx: EguiContexts,
    mut app_exit_events: EventWriter<AppExit>,
    actions: EventWriter<MenuActionEvent>,
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
}

fn add_main_menu(
    menu_state: &State<MenuState>,
    set_menu_state: &mut NextState<MenuState>,
    egui_ctx: &mut EguiContexts,
    app_exit_events: &mut EventWriter<AppExit>,
    mut actions: EventWriter<MenuActionEvent>,
) {
    SidePanel::left("left_panel")
        .resizable(false)
        .default_width(SIDE_BAR_WIDTH)
        .frame(Frame::new().fill(Color32::from_rgba_premultiplied(0, 0, 0, 50)))
        .show(egui_ctx.ctx_mut(), |ui| {
            // Title
            ui.add_sized(
                [SIDE_BAR_WIDTH, 60.0],
                Label::new(RichText::new(TITLE).heading()),
            );

            if add_menu_button("Play", ui).clicked() {
                actions.send(MenuActionEvent::EnterNewGameMenu);
            }

            if add_menu_button("Map Editor", ui).clicked() {
                actions.send(MenuActionEvent::LeaveMenu(GameState::MapEditor));
            }

            if add_menu_button("Settings", ui).clicked() {
                match **menu_state {
                    MenuState::Settings => set_menu_state.set(MenuState::Main),
                    _ => set_menu_state.set(MenuState::Settings),
                }
            }

            if add_menu_button("Quit", ui).clicked() {
                app_exit_events.send(AppExit::Success);
            }
        });
}

fn add_menu_button(text: &str, ui: &mut egui::Ui) -> Response {
    ui.add_sized([SIDE_BAR_WIDTH, 60.0], egui::Button::new(text).frame(false))
}

fn add_settings(egui_ctx: &mut EguiContexts, mut settings: ResMut<crate::user::Settings>) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(ui.available_height());
        ScrollArea::vertical().show(ui, |ui| {
            let volume_silder = egui::Slider::new(&mut settings.volume.0, 0..=100)
                .show_value(false)
                .clamping(SliderClamping::Always);
            add_row("Volume", volume_silder, ui);
        });
    });
}
