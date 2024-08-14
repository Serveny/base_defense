use ::bevy_egui::{
    egui::{self, style::Selection, Color32, Stroke},
    EguiPlugin,
};
use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;
use user::Settings;
use utils::GameState;

//use bevy_editor_pls::*;
use bevy_egui::EguiContexts;
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
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            viewport_origin: Vec2::new(0., 0.),
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 100.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        ..default()
    });
}

const fn font() -> &'static [u8; 78628] {
    #[cfg(windows)]
    return include_bytes!("..\\assets\\fonts\\Quicksand-Regular.ttf");

    #[cfg(unix)]
    include_bytes!("../assets/fonts/Quicksand-Regular.ttf")
}

fn setup_egui(mut egui_ctx: EguiContexts) {
    let mut fonts = egui::FontDefinitions::default();
    let font = font();

    fonts.font_data.insert(
        "Quicksand-Regular".to_owned(),
        egui::FontData::from_static(font),
    );
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
        ..default()
    });
}
