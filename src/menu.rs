pub struct MenuPlugin;
use crate::{
    utils::{add_row, GameState},
    TITLE,
};
use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{
        self, CentralPanel, Color32, FontDefinitions, Frame, Label, Response, RichText, ScrollArea,
        SidePanel,
    },
    EguiContext,
};

const SIDE_BAR_WIDTH: f32 = 300.0;

// State used for the current menu screen
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Main,
    Settings,
    Disabled,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(startup_menu))
            .add_state(MenuState::Disabled)
            .add_startup_system(setup_fonts)
            .add_startup_system(configure_visuals);
    }
}

pub fn setup_fonts(mut egui_ctx: ResMut<EguiContext>) {
    let mut fonts = FontDefinitions::default();

    #[cfg(windows)]
    let font = include_bytes!("..\\assets\\fonts\\Quicksand-Regular.ttf");

    #[cfg(unix)]
    let font = include_bytes!("../assets/fonts/Quicksand-Regular.ttf");

    fonts.font_data.insert(
        "Quicksand-Regular".to_owned(),
        egui::FontData::from_static(font),
    );
    // Put Quicksand-Regular first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "Quicksand-Regular".to_owned());

    for (_text_style, data) in fonts.font_data.iter_mut() {
        data.tweak.scale = 2.;
    }
    egui_ctx.ctx_mut().set_fonts(fonts);
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 10.0.into(),
        ..Default::default()
    });
}

fn startup_menu(
    mut game_state: ResMut<State<GameState>>,
    mut menu_state: ResMut<State<MenuState>>,
    mut egui_ctx: ResMut<EguiContext>,
    mut app_exit_events: EventWriter<AppExit>,
    mut settings: ResMut<crate::user::Settings>,
) {
    add_main_menu(
        &mut game_state,
        &mut menu_state,
        &mut egui_ctx,
        &mut app_exit_events,
    );

    match *menu_state.current() {
        MenuState::Settings => add_settings(&mut egui_ctx, &mut settings),
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
                leave_menu(GameState::Game, menu_state, game_state);
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

fn add_settings(egui_ctx: &mut ResMut<EguiContext>, settings: &mut ResMut<crate::user::Settings>) {
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

fn leave_menu(
    to: GameState,
    menu_state: &mut ResMut<State<MenuState>>,
    game_state: &mut ResMut<State<GameState>>,
) {
    menu_state.set(MenuState::Disabled).unwrap();
    game_state.set(to).unwrap();
}
