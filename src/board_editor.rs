use crate::{
    board::{Board, Tile},
    utils::{
        add_error_box, add_ok_cancel_row, add_popup_window, add_row, despawn_all_of,
        get_all_boards_in_folder, save_board_to_file, GameState,
    },
};
use bevy::{prelude::*, sprite::Anchor, window::WindowResized};
use bevy_egui::{
    egui::{self, Label, RadioButton, Response, SidePanel, TextEdit, TopBottomPanel, Ui},
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

#[derive(Component, Debug)]
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

struct BoardEditorState {
    current_map: Board,
}

impl Default for BoardEditorState {
    fn default() -> Self {
        Self {
            current_map: Board::default(),
        }
    }
}

#[derive(Default)]
struct NewBoardWindow {
    width: u8,
    height: u8,
}

struct LoadBoardWindow {
    boards: Vec<Board>,
    err_text: Option<String>,
}

impl Default for LoadBoardWindow {
    fn default() -> Self {
        match get_all_boards_in_folder() {
            Ok(boards) => Self {
                boards,
                err_text: None,
            },
            Err(err) => Self {
                boards: Vec::new(),
                err_text: Some(err.to_string()),
            },
        }
    }
}

#[derive(Default)]
struct SaveBoardWindow {
    map_file_name: String,
    err_text: Option<String>,
}

enum PopupWindows {
    Load(LoadBoardWindow),
    Save(SaveBoardWindow),
    None,
}

impl PopupWindows {
    fn is_open(&self) -> bool {
        match *self {
            PopupWindows::None => false,
            _ => true,
        }
    }
}

// This plugin will contain the map editor
pub struct BoardEditorPlugin;

impl Plugin for BoardEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(SettileState::TowerGround)
            .add_system_set(SystemSet::on_enter(GameState::MapEditor).with_system(editor_setup))
            .add_system_set(
                SystemSet::on_update(GameState::MapEditor)
                    .with_system(add_top_menu_bar)
                    .with_system(add_side_tool_bar)
                    .with_system(on_resize)
                    .with_system(editor_click_actions)
                    .with_system(add_load_board_window)
                    .with_system(add_save_board_window),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MapEditor)
                    .with_system(despawn_all_of::<OnBoardEditorScreen>)
                    .with_system(clean_up_editor),
            );
    }
}
fn editor_setup(mut commands: Commands, windows: Res<Windows>) {
    let editor_state = BoardEditorState::default();
    spawn_tiles(&mut commands, &windows, &editor_state.current_map);

    commands.insert_resource(editor_state);
    commands.insert_resource(PopupWindows::None);
}

fn add_top_menu_bar(
    mut egui_ctx: ResMut<EguiContext>,
    mut game_state: ResMut<State<GameState>>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<PopupWindows>,
) {
    TopBottomPanel::top("map_editor_top_bar").show(egui_ctx.ctx_mut(), |ui| {
        ui.set_height(TOP_BAR_HEIGHT_PX);
        ui.horizontal(|ui| {
            if add_top_bar_button("Back", ui).clicked() {
                game_state.set(GameState::Menu).unwrap();
            }

            ui.add_space(LEFT_BAR_WIDTH_PX - 80.);

            if add_top_bar_button("Load", ui).clicked() {
                *popup = match *popup {
                    PopupWindows::Load(_) => PopupWindows::None,
                    _ => PopupWindows::Load(LoadBoardWindow::default()),
                }
            }

            if add_top_bar_button("Save", ui).clicked() {
                *popup = match *popup {
                    PopupWindows::Save(_) => PopupWindows::None,
                    _ => PopupWindows::Save(SaveBoardWindow::default()),
                }
            }

            if add_top_bar_button("New", ui).clicked() {
                state.current_map = Board::default();
            }
        });
    });
}

fn add_save_board_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<PopupWindows>,
) {
    let mut is_close = false;
    if let PopupWindows::Save(save_win) = &mut *popup {
        add_popup_window(&mut egui_ctx, "Save map", |ui| {
            add_row(
                "Map name",
                TextEdit::singleline(&mut save_win.map_file_name).margin(egui::Vec2::new(10., 16.)),
                ui,
            );
            ui.add_space(10.);

            // Ok/Cancel Buttons
            ui.add_space(10.);
            let (is_ok, is_cancel) = add_ok_cancel_row(ui);

            if is_ok {
                save_win.err_text = None;
                state.current_map.name = save_win.map_file_name.clone();
                match save_board_to_file(&save_win.map_file_name, &state.current_map) {
                    Ok(()) => is_close = true,
                    Err(error) => add_error_box(&error.to_string(), ui),
                }
            } else if is_cancel {
                is_close = true;
            }
            // Error container
            if let Some(err_text) = &save_win.err_text {
                add_error_box(err_text, ui);
                add_error_box(err_text, ui);
            }
        });
    }
    if is_close {
        *popup = PopupWindows::None;
    }
}

fn add_load_board_window(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<PopupWindows>,
    mut editor_tiles: Query<Entity, With<EditorTile>>,
    windows: Res<Windows>,
) {
    let mut is_close = false;
    if let PopupWindows::Load(load_win) = &mut *popup {
        add_popup_window(&mut egui_ctx, "Load map", |ui| {
            egui::containers::ScrollArea::vertical().show(ui, |ui| {
                for board in &load_win.boards {
                    if ui
                        .add_sized(
                            [400., 60.],
                            bevy_egui::egui::widgets::Button::new(&board.name),
                        )
                        .clicked()
                    {
                        state.current_map = board.clone();
                        repaint(&mut commands, &mut editor_tiles, &windows, &state);
                        is_close = true;
                    }
                }
            });
            ui.add_space(10.);
            if ui
                .add_sized([400., 60.], bevy_egui::egui::widgets::Button::new("Cancel"))
                .clicked()
            {
                is_close = true;
            }
            if let Some(err_text) = &load_win.err_text {
                add_error_box(err_text, ui);
                add_error_box(err_text, ui);
            }
        });
    }
    if is_close {
        *popup = PopupWindows::None;
    }
}

fn repaint(
    commands: &mut Commands,
    editor_tiles: &mut Query<Entity, With<EditorTile>>,
    windows: &Windows,
    state: &BoardEditorState,
) {
    for entity in editor_tiles.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    spawn_tiles(commands, windows, &state.current_map);
}

fn add_top_bar_button(text: &str, ui: &mut Ui) -> Response {
    ui.add_sized(
        [60., TOP_BAR_HEIGHT_PX],
        egui::Button::new(text).frame(false),
    )
}

fn add_side_tool_bar(
    mut egui_ctx: ResMut<EguiContext>,
    mut set_tile_state: ResMut<State<SettileState>>,
) {
    SidePanel::left("map_editor_left_bar")
        .resizable(false)
        .default_width(LEFT_BAR_WIDTH_PX)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.add_sized([LEFT_BAR_WIDTH_PX - 20., 40.], Label::new("tile type"));
            add_tile_radio_button(&mut set_tile_state, SettileState::TowerGround, "Tower", ui);
            add_tile_radio_button(
                &mut set_tile_state,
                SettileState::BuildingGround,
                "Building",
                ui,
            );
            add_tile_radio_button(&mut set_tile_state, SettileState::Road, "Road", ui);
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

fn spawn_tiles(commands: &mut Commands, windows: &Windows, board: &Board) {
    let rs_params = TileResizeParams::new(windows, board);
    for (x, row) in board.tiles.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            spawn_tile(tile, x, y, &rs_params, commands);
        }
    }
}

fn spawn_tile(
    tile: &Tile,
    x: usize,
    y: usize,
    rs_params: &TileResizeParams,
    commands: &mut Commands,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(rs_params.tile_inner_size),
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
}

fn get_tile_color(tile: &Tile) -> Color {
    match tile {
        Tile::TowerGround(_) => Color::GOLD,
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

fn clean_up_editor(mut commands: Commands) {
    commands.remove_resource::<BoardEditorState>();
    commands.remove_resource::<PopupWindows>();
}

fn on_resize(
    events: EventReader<WindowResized>,
    editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
    windows: Res<Windows>,
    state: Res<BoardEditorState>,
) {
    if !events.is_empty() {
        resize_tiles(windows, editor_tiles, state);
    }
}

fn resize_tiles(
    windows: Res<Windows>,
    mut editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
    state: Res<BoardEditorState>,
) {
    let rs_params = TileResizeParams::new(&windows, &state.current_map);
    editor_tiles.for_each_mut(|(mut sprite, mut transform, tile)| {
        sprite.custom_size = Some(rs_params.tile_inner_size);
        transform.translation = Vec3::new(
            rs_params.board_start_x + (tile.pos.x as f32 * rs_params.tile_size),
            rs_params.board_start_y - (tile.pos.y as f32 * rs_params.tile_size),
            0.,
        );
    });
}

struct TileResizeParams {
    tile_inner_size: Vec2,
    tile_size: f32,
    board_start_x: f32,
    board_start_y: f32,
}

impl TileResizeParams {
    fn new(windows: &Windows, board: &Board) -> Self {
        let window = windows.get_primary().unwrap();
        let board_width_px = window.width() - LEFT_BAR_WIDTH_PX;
        let board_height_px = window.height() - TOP_BAR_HEIGHT_PX;

        // the tiles are quadratic, so use the smaller size
        let tile_size = get_tile_size_px(board_width_px, board_height_px, board);
        let tile_inner_size = Vec2::new(tile_size - 10., tile_size - 10.);

        // Think from the middle of the sceen
        let board_start_x = (LEFT_BAR_WIDTH_PX - board_width_px) / 2.;
        let board_start_y = (board_height_px - TOP_BAR_HEIGHT_PX) / 2.;

        Self {
            tile_inner_size,
            tile_size,
            board_start_x,
            board_start_y,
        }
    }
}

fn editor_click_actions(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    popups: Res<PopupWindows>,
    set_tile_state: Res<State<SettileState>>,
    mut state: ResMut<BoardEditorState>,
    mut editor_tiles: Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
) {
    if popups.is_open() {
        return;
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let tile = settile_state_to_tile(set_tile_state.current().clone());
        set_tile(&windows, &mut state.current_map, &mut editor_tiles, tile);
    } else if mouse_button_input.just_pressed(MouseButton::Right) {
        set_tile(
            &windows,
            &mut state.current_map,
            &mut editor_tiles,
            Tile::Empty,
        );
    }
}

fn set_tile(
    windows: &Res<Windows>,
    board: &mut Board,
    editor_tiles: &mut Query<(&mut Sprite, &mut Transform, &EditorTile), With<EditorTile>>,
    tile_to: Tile,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor_pos) = window.cursor_position() {
        for (mut sprite, transform, tile) in editor_tiles.iter_mut() {
            if is_hover(
                cursor_pos_to_transform_pos(cursor_pos, &window),
                &sprite,
                &transform,
            ) {
                sprite.color = get_tile_color(&tile_to);
                board.tiles[tile.pos.x as usize][tile.pos.y as usize] = tile_to;
                break;
            }
        }
    }
}

fn is_hover(cursor_pos: Vec2, sprite: &Sprite, transform: &Transform) -> bool {
    //println!("is_hover: {} | {}", cursor_pos, transform.translation);
    if let Some(size) = sprite.custom_size {
        cursor_pos.x >= transform.translation.x
            && cursor_pos.x <= transform.translation.x + size.x
            && cursor_pos.y >= transform.translation.y - size.y
            && cursor_pos.y <= transform.translation.y
    } else {
        false
    }
}

fn cursor_pos_to_transform_pos(cursor_pos: Vec2, window: &Window) -> Vec2 {
    Vec2::new(
        cursor_pos.x - (window.width() / 2.),
        cursor_pos.y - (window.height() / 2.),
    )
}

fn settile_state_to_tile(settile_state: SettileState) -> Tile {
    match settile_state {
        SettileState::TowerGround => Tile::TowerGround(None),
        SettileState::BuildingGround => Tile::BuildingGround(None),
        SettileState::Road => Tile::Road,
    }
}
