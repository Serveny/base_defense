use assets::StandardAssets;
use bevy::{
    prelude::*,
    render::camera::{Camera2d, ScalingMode},
};
use bevy_asset_loader::AssetLoader;
use bevy_egui::{
    egui::{self, style::Selection, Color32, Stroke},
    EguiContext, EguiPlugin,
};
use bevy_prototype_lyon::plugin::ShapePlugin;
use board::Board;
use user::Settings;
use utils::GameState;

#[cfg(feature = "debug")]
//use bevy_editor_pls::*;
use bevy_inspector_egui::*;

mod assets;
mod board;
mod board_editor;
mod game;
mod main_menu;
mod splash;
mod user;
mod utils;

type CamQuery<'w, 's, 'a> = Query<'w, 's, &'a mut OrthographicProjection, With<Camera2d>>;

const TITLE: &str = "Base Defense";
const BACKGROUND_COLOR: Color = Color::rgba(35.0 / 255.0, 33.0 / 255.0, 38.0 / 255.0, 15.0);
//Color::rgb(50. / 255., 50. / 255., 44. / 255.);
// const TEXT_COLOR: Color = Color::rgb(232.0 / 255.0, 230.0 / 255.0, 227.0 / 255.0);
// const FONT_PATH: &str = "fonts/Quicksand-Regular.ttf";

fn main() {
    let mut app = App::new();
    let window_setup = WindowDescriptor {
        title: TITLE.to_string(),
        //position: Some(Vec2::new(3000., 200.)),
        ..Default::default()
    };

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(window_setup)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(bevy_screen_diags::ScreenDiagsPlugin)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(board_editor::BoardEditorPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    //   app.add_plugin(EditorPlugin)
    //      .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    //      .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin);

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
    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scaling_mode = ScalingMode::None;
    commands.spawn_bundle(cam);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn zoom_cam_to_board(board: &Board, mut cam_query: CamQuery, windows: &Windows) {
    let win = windows.get_primary().unwrap();
    let mut cam = cam_query.single_mut();
    let margin = cam_margin(board, win);
    println!("{:?}", margin);
    cam.left = -margin.0;
    cam.bottom = -margin.1;
    cam.right = board.width as f32 + margin.0;
    cam.top = board.height as f32 + margin.1;
}

fn cam_margin(board: &Board, win: &Window) -> (f32, f32) {
    let b_w = board.width as f32;
    let b_h = board.height as f32;

    let tile_width_px = win.width() / b_w as f32;
    let tile_height_px = win.height() / b_h as f32;

    if tile_height_px > tile_width_px {
        (0., ((win.height() / tile_width_px) - b_h) / 2.)
    } else {
        (((win.width() / tile_height_px) - b_w) / 2., 0.)
    }
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
        selection: Selection {
            bg_fill: Color32::from_rgb(54, 241, 205),
            stroke: Stroke {
                width: 2.,
                color: Color32::WHITE,
            },
        },
        ..Default::default()
    });
}
