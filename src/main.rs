use ::bevy_egui::{
    egui::{self, style::Selection, Color32, Stroke},
    EguiPlugin,
};
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use user::Settings;
use utils::GameState;

//use bevy_editor_pls::*;
use bevy_egui::{
    egui::epaint::text::{FontInsert, InsertFontFamily},
    EguiContexts,
};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod assets;
mod board;
mod board_editor;
mod game;
mod main_menu;
mod splash;
mod user;
mod utils;

type CamQuery<'w, 's, 'a> = Query<'w, 's, (&'a Camera, &'a GlobalTransform), With<Camera2d>>;
type CamMutQuery<'w, 's, 'a> = Query<'w, 's, &'a mut OrthographicProjection, With<Camera2d>>;

const TITLE: &str = "Base Defense";
const BACKGROUND_COLOR: Color = Color::srgba(35.0 / 255.0, 33.0 / 255.0, 38.0 / 255.0, 15.0);
//Color::rgb(50. / 255., 50. / 255., 44. / 255.);
// const TEXT_COLOR: Color = Color::rgb(232.0 / 255.0, 230.0 / 255.0, 227.0 / 255.0);
// const FONT_PATH: &str = "fonts/Quicksand-Regular.ttf";

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: TITLE.to_string(),
                resolution: (1200.0, 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EguiPlugin,
            ShapePlugin,
            splash::SplashPlugin,
            main_menu::MainMenuPlugin,
            game::GamePlugin,
            board_editor::BoardEditorPlugin,
        ));

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::default());
    //   app.add_plugin(EditorPlugin)
    //      .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    //      .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin);

    //AssetLoader::new(GameState::Splash)
    //.continue_to_state(GameState::Menu)
    //.with_collection::<AssetServer>()
    //.build(&mut app);

    app.insert_resource(Settings::new())
        .init_state::<GameState>()
        .add_systems(Startup, (setup_cameras, setup_egui))
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            viewport_origin: Vec2::new(0., 0.),
            ..OrthographicProjection::default_2d()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 100.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
    ));
}

const fn font_quicksand_regular() -> &'static [u8; 78628] {
    #[cfg(windows)]
    return include_bytes!("..\\assets\\fonts\\Quicksand-Regular.ttf");

    #[cfg(unix)]
    include_bytes!("../assets/fonts/Quicksand-Regular.ttf")
}

fn add_font(ctx: &egui::Context, name: &str, font: &'static [u8]) {
    ctx.add_font(FontInsert::new(
        name,
        egui::FontData::from_static(font),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

fn setup_egui(mut egui_ctx: EguiContexts) {
    add_font(
        egui_ctx.ctx_mut(),
        "Quicksand-Regular",
        font_quicksand_regular(),
    );

    //Visuals
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_corner_radius: 10.0.into(),
        selection: Selection {
            bg_fill: Color32::from_rgb(54, 241, 205),
            stroke: Stroke {
                width: 2.,
                color: Color32::WHITE,
            },
        },
        ..default()
    });
}
