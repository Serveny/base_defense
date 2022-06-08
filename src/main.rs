use assets::StandardAssets;
use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_prototype_lyon::plugin::ShapePlugin;
use user::Settings;
use utils::GameState;

#[cfg(feature = "debug")]
use bevy_editor_pls::*;

mod assets;
mod board;
mod board_editor;
mod game;
mod main_menu;
mod splash;
mod user;
mod utils;

const TITLE: &str = "Base Defense";
const BACKGROUND_COLOR: Color = Color::rgba(35.0 / 255.0, 33.0 / 255.0, 38.0 / 255.0, 15.0);
// const TEXT_COLOR: Color = Color::rgb(232.0 / 255.0, 230.0 / 255.0, 227.0 / 255.0);
// const FONT_PATH: &str = "fonts/Quicksand-Regular.ttf";

fn main() {
    let mut app = App::new();
    let window_setup = WindowDescriptor {
        title: TITLE.to_string(),
        position: Some(Vec2::new(3000., 200.)),
        ..Default::default()
    };

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(window_setup)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(board_editor::BoardEditorPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(EditorPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin);

    AssetLoader::new(GameState::Splash)
        .continue_to_state(GameState::Menu)
        .with_collection::<StandardAssets>()
        .build(&mut app);

    app.insert_resource(Settings::new())
        .add_state(GameState::Splash)
        .add_startup_system(setup_cameras)
        .add_startup_system(setup_egui)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_egui(mut egui_ctx: ResMut<EguiContext>) {
    // Fonts
    let mut fonts = egui::FontDefinitions::default();

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

    //Visuals
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 10.0.into(),
        ..Default::default()
    });
}
