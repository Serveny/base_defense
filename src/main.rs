use ::bevy_egui::{
    egui::{self, style::Selection, Color32, FontId, Stroke, TextStyle},
    EguiPlugin,
};
use bevy::{prelude::*, window::WindowResolution};
#[cfg(debug_assertions)]
use bevy_inspector_egui::{bevy_inspector, DefaultInspectorConfigPlugin};
use bevy_prototype_lyon::plugin::ShapePlugin;
use user::Settings;
use utils::GameState;

//use bevy_editor_pls::*;
use bevy_egui::{
    egui::epaint::text::{FontInsert, InsertFontFamily},
    EguiContexts, EguiStartupSet,
};
#[cfg(debug_assertions)]
use bevy_egui::{EguiContext, EguiPrimaryContextPass, PrimaryEguiContext};

mod assets;
mod board;
mod board_editor;
mod game;
mod main_menu;
mod splash;
mod user;
mod utils;

type CamQuery<'w, 's, 'a> = Query<'w, 's, (&'a Camera, &'a GlobalTransform), With<Camera2d>>;
type CamMutQuery<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Projection, &'a mut Camera, &'a mut Transform), With<Camera2d>>;

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
                resolution: WindowResolution::new(1200, 600),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            EguiPlugin::default(),
            ShapePlugin,
            splash::SplashPlugin,
            main_menu::MainMenuPlugin,
            game::GamePlugin,
            board_editor::BoardEditorPlugin,
        ));

    #[cfg(debug_assertions)]
    app.add_plugins(DefaultInspectorConfigPlugin)
        .add_systems(EguiPrimaryContextPass, world_inspector_ui);
    //   app.add_plugin(EditorPlugin)
    //      .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    //      .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin);

    //AssetLoader::new(GameState::Splash)
    //.continue_to_state(GameState::Menu)
    //.with_collection::<AssetServer>()
    //.build(&mut app);

    app.insert_resource(Settings::new())
        .init_state::<GameState>()
        .add_systems(
            PreStartup,
            setup_cameras.before(EguiStartupSet::InitContexts),
        )
        .add_systems(Startup, setup_egui)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            viewport_origin: Vec2::new(0., 0.),
            ..OrthographicProjection::default_2d()
        }),
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
    let Ok(ctx) = egui_ctx.ctx_mut() else {
        error!("Failed to get egui context");
        return;
    };
    add_font(ctx, "Quicksand-Regular", font_quicksand_regular());

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(34., egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(20., egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(20., egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Small,
        FontId::new(16., egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(19., egui::FontFamily::Monospace),
    );
    style.spacing.button_padding = egui::vec2(16., 10.);
    style.spacing.item_spacing = egui::vec2(10., 10.);
    ctx.set_style(style);

    //Visuals
    ctx.set_visuals(egui::Visuals {
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

#[cfg(debug_assertions)]
fn world_inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();
    let ctx = egui_context.get_mut();
    let original_style = (*ctx.style()).clone();
    let mut inspector_style = original_style.clone();

    inspector_style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(18., egui::FontFamily::Proportional),
    );
    inspector_style.text_styles.insert(
        TextStyle::Body,
        FontId::new(12., egui::FontFamily::Proportional),
    );
    inspector_style.text_styles.insert(
        TextStyle::Button,
        FontId::new(12., egui::FontFamily::Proportional),
    );
    inspector_style.text_styles.insert(
        TextStyle::Small,
        FontId::new(10., egui::FontFamily::Proportional),
    );
    inspector_style.spacing.item_spacing = egui::vec2(4., 4.);
    inspector_style.spacing.button_padding = egui::vec2(6., 3.);

    ctx.set_style(inspector_style);
    egui::Window::new("World Inspector")
        .default_size(egui::vec2(320., 180.))
        .show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
    ctx.set_style(original_style);
}
