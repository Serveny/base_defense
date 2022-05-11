use crate::{
    board::{Board, Tile},
    utils::{despawn_screen, GameState},
};
use bevy::{prelude::*, sprite::Anchor, window::WindowResized};
use bevy_egui::{
    egui::{self, Label, RadioButton, Response, SidePanel, TopBottomPanel, Ui},
    EguiContext,
};

#[derive(Component)]
struct OnBoardEditorScreen;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum SettileState {
    TowerGround,
    BuildingGround,
    Road,
}

#[derive(Component)]
struct EditorTile {
    pos: UVec2,
}

impl EditorTile {
    fn new(pos: UVec2) -> Self {
        Self { pos }
    }
}
const TOP_BAR_HEIGHT_PX: f32 = 40.0;
const LEFT_BAR_WIDTH_PX: f32 = 140.0;

struct BoardEditorCurrentMap(Board);

// This plugin will contain the map editor
pub struct BoardEditorPlugin;

impl Plugin for BoardEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(SettileState::TowerGround)
            .insert_resource(BoardEditorCurrentMap(Board::default()))
            .add_system_set(
                SystemSet::on_enter(GameState::MapEditor)
                    .with_system(spawn_tiles)
                    .with_system(resize_tiles),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MapEditor)
                    .with_system(update_ui)
                    .with_system(on_resize),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MapEditor)
                    .with_system(despawn_screen::<OnBoardEditorScreen>)
                    .with_system(clean_up_editor),
            );
    }
}

fn update_ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut set_tile_state: ResMut<State<SettileState>>,
    mut game_state: ResMut<State<GameState>>,
    mut current_map: ResMut<BoardEditorCurrentMap>,
) {
    add_top_menu_bar(&mut egui_ctx, &mut game_state, &mut current_map);
    add_side_tool_bar(&mut egui_ctx, &mut set_tile_state);
}

fn add_top_menu_bar(
    egui_ctx: &mut ResMut<EguiContext>,
    game_state: &mut ResMut<State<GameState>>,
    current_map: &mut ResMut<BoardEditorCurrentMap>,
) {
    TopBottomPanel::top("map_editor_top_bar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(TOP_BAR_HEIGHT_PX);
        ui.horizontal(|ui| {
            if add_top_bar_button("Back", ui).clicked() {
                game_state.set(GameState::Menu).unwrap();
            }

            ui.add_space(LEFT_BAR_WIDTH_PX - 80.);

            if add_top_bar_button("Save", ui).clicked() {}

            if add_top_bar_button("Load", ui).clicked() {}

            if add_top_bar_button("New", ui).clicked() {
                **current_map = BoardEditorCurrentMap(Board::default());
            }
        });
    });
}

fn add_top_bar_button(text: &str, ui: &mut Ui) -> Response {
    ui.add_sized(
        [60., TOP_BAR_HEIGHT_PX],
        egui::Button::new(text).frame(false),
    )
}

fn add_side_tool_bar(
    egui_ctx: &mut ResMut<EguiContext>,
    set_tile_state: &mut ResMut<State<SettileState>>,
) {
    SidePanel::left("map_editor_left_bar")
        .resizable(false)
        .default_width(LEFT_BAR_WIDTH_PX)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add_sized([LEFT_BAR_WIDTH_PX - 20., 40.], Label::new("Set tile"));
            add_tile_radio_button(set_tile_state, SettileState::TowerGround, "Tower", ui);
            add_tile_radio_button(set_tile_state, SettileState::BuildingGround, "Building", ui);
            add_tile_radio_button(set_tile_state, SettileState::Road, "Road", ui);
        });
}

fn add_tile_radio_button(
    current_state: &mut ResMut<State<SettileState>>,
    state: SettileState,
    text: &str,
    ui: &mut egui::Ui,
) {
    if ui
        .add(RadioButton::new(*current_state.current() == state, text))
        .clicked()
    {
        current_state.set(state).unwrap();
    }
}

fn spawn_tiles(
    mut commands: Commands,
    windows: Res<Windows>,
    current_map: Res<BoardEditorCurrentMap>,
) {
    let rs_params = TileResizeParams::new(windows, &current_map);
    current_map.0.tiles.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, tile)| {
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(rs_params.tile_size_vec2),
                        color: get_tile_color(tile),
                        anchor: Anchor::TopLeft,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(
                            rs_params.board_start_x + (x as f32 * rs_params.tile_size),
                            rs_params.board_start_y - (y as f32 * rs_params.tile_size),
                            0.,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(EditorTile::new(UVec2::new(x as u32, y as u32)))
                .insert(OnBoardEditorScreen);
        });
    });
}

fn get_tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::TowerGround(_) => Color::ALICE_BLUE,
        Tile::BuildingGround(_) => Color::ANTIQUE_WHITE,
        Tile::Road => Color::AQUAMARINE,
        Tile::Empty => Color::AZURE,
    }
}

fn get_tile_size_px(board_width_px: f32, board_height_px: f32, board: &Board) -> f32 {
    let tile_width_px = board_width_px / board.width as f32;
    let tile_height_px = board_height_px / board.height as f32;

    if tile_height_px > tile_width_px {
        tile_width_px
    } else {
        tile_height_px
    }
}

fn clean_up_editor(mut current_map: ResMut<BoardEditorCurrentMap>) {
    *current_map = BoardEditorCurrentMap(Board::default());
}

fn on_resize(
    events: EventReader<WindowResized>,
    editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
    windows: Res<Windows>,
    current_map: Res<BoardEditorCurrentMap>,
) {
    if !events.is_empty() {
        resize_tiles(windows, editor_tiles, current_map);
    }
}

fn resize_tiles(
    windows: Res<Windows>,
    mut editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
    current_map: Res<BoardEditorCurrentMap>,
) {
    let rs_params = TileResizeParams::new(windows, &current_map);
    editor_tiles.for_each_mut(|(mut sprite, mut transform, tile)| {
        sprite.custom_size = Some(rs_params.tile_size_vec2);
        transform.translation = Vec3::new(
            rs_params.board_start_x + (tile.pos.x as f32 * rs_params.tile_size),
            rs_params.board_start_y - (tile.pos.y as f32 * rs_params.tile_size),
            0.,
        );
    });
}

struct TileResizeParams {
    tile_size_vec2: Vec2,
    tile_size: f32,
    board_start_x: f32,
    board_start_y: f32,
}

impl TileResizeParams {
    fn new(windows: Res<Windows>, map: &BoardEditorCurrentMap) -> Self {
        let window = windows.get_primary().unwrap();
        let board_width_px = window.width() - LEFT_BAR_WIDTH_PX;
        let board_height_px = window.height() - TOP_BAR_HEIGHT_PX;
        let tile_size = get_tile_size_px(board_width_px, board_height_px, &map.0);
        let tile_size_vec2 = Vec2::new(tile_size - 10., tile_size - 10.);

        // Think from the middle of the sceen
        let board_start_x = (LEFT_BAR_WIDTH_PX - board_width_px) / 2.;
        let board_start_y = (board_height_px - TOP_BAR_HEIGHT_PX) / 2.;

        Self {
            tile_size_vec2,
            tile_size,
            board_start_x,
            board_start_y,
        }
    }
}
