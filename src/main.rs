use assets::StandardAssets;
use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_egui::EguiPlugin;
use user::Settings;
use utils::GameState;

#[cfg(feature = "debug")]
use bevy_editor_pls::*;

mod assets;
mod board;
mod board_editor;
mod game;
mod menu;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
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
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
