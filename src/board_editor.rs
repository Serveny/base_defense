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

#[derive(Default)]
struct BoardEditorState {
    current_map: Board,
    err_text: Option<String>,
}

struct NewBoardWindow {
    width: u8,
    height: u8,
}

impl Default for NewBoardWindow {
    fn default() -> Self {
        Self {
            width: 10,
            height: 6,
        }
    }
}

struct EditBoardWindow {
    width: u8,
    height: u8,
}

impl EditBoardWindow {
    fn new(board: &Board) -> Self {
        Self {
            width: board.width,
            height: board.height,
        }
    }
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
    New(NewBoardWindow),
    Edit(EditBoardWindow),
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
                    .with_system(add_top_menu_bar.before(add_side_tool_bar))
                    .with_system(add_side_tool_bar)
                    .with_system(on_resize)
                    .with_system(editor_click_actions)
                    .with_system(add_load_board_window)
                    .with_system(add_save_board_window)
                    .with_system(add_new_board_window)
                    .with_system(add_edit_board_window),
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
    mut popup: ResMut<PopupWindows>,
    state: Res<BoardEditorState>,
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
                *popup = match *popup {
                    PopupWindows::New(_) => PopupWindows::None,
                    _ => PopupWindows::New(NewBoardWindow::default()),
                }
            }

            if add_top_bar_button("Edit", ui).clicked() {
                *popup = match *popup {
                    PopupWindows::Edit(_) => PopupWindows::None,
                    _ => PopupWindows::Edit(EditBoardWindow::new(&state.current_map)),
                }
            }

            if let Some(err_text) = &state.err_text {
                ui.add_space(LEFT_BAR_WIDTH_PX - 80.);
                add_error_box(err_text, ui);
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

fn add_new_board_window(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<PopupWindows>,
    mut editor_tiles: Query<Entity, With<EditorTile>>,
    windows: Res<Windows>,
) {
    let mut is_close = false;
    if let PopupWindows::New(new_win) = &mut *popup {
        let (is_ok, is_cancel) =
            add_new_edit_popup(egui_ctx, &mut new_win.width, &mut new_win.height, "New map");

        if is_ok {
            state.current_map = Board::empty(new_win.width, new_win.height);
            repaint(&mut commands, &mut editor_tiles, &windows, &state);
            is_close = true;
        } else if is_cancel {
            is_close = true;
        }
    }
    if is_close {
        *popup = PopupWindows::None;
    }
}

fn add_edit_board_window(
    mut commands: Commands,
    egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<BoardEditorState>,
    mut popup: ResMut<PopupWindows>,
    mut editor_tiles: Query<Entity, With<EditorTile>>,
    windows: Res<Windows>,
) {
    let mut is_close = false;
    if let PopupWindows::Edit(edit_win) = &mut *popup {
        let (is_ok, is_cancel) = add_new_edit_popup(
            egui_ctx,
            &mut edit_win.width,
            &mut edit_win.height,
            "Edit size",
        );

        if is_ok {
            edit_board(&mut state.current_map, edit_win.width, edit_win.height);
            repaint(&mut commands, &mut editor_tiles, &windows, &state);
            is_close = true;
        } else if is_cancel {
            is_close = true;
        }
    }
    if is_close {
        *popup = PopupWindows::None;
    }
}

fn add_new_edit_popup(
    mut egui_ctx: ResMut<EguiContext>,
    width: &mut u8,
    height: &mut u8,
    title: &str,
) -> (bool, bool) {
    let (mut is_ok, mut is_cancel) = (false, false);
    add_popup_window(&mut egui_ctx, title, |ui| {
        // Width
        let width_silder = egui::Slider::new(width, 4..=32)
            .show_value(true)
            .clamp_to_range(true);
        add_row("Width", width_silder, ui);

        // Height
        let height_silder = egui::Slider::new(height, 4..=32)
            .show_value(true)
            .clamp_to_range(true);
        add_row("Height", height_silder, ui);

        // Ok/Cancel Buttons
        ui.add_space(10.);
        (is_ok, is_cancel) = add_ok_cancel_row(ui);
    });
    (is_ok, is_cancel)
}

fn edit_board(board: &mut Board, new_width: u8, new_heigth: u8) {
    // Add/reduce width
    if new_width > board.width {
        let to_add = new_width - board.width;
        for row in &mut board.tiles {
            for _ in 0..to_add {
                row.push(Tile::Empty);
            }
        }
    } else if new_width < board.width {
        let to_del = board.width - new_width;
        for row in &mut board.tiles {
            for _ in 0..to_del {
                row.pop();
            }
        }
    }

    // Add/reduce height
    if new_heigth > board.height {
        let to_add = new_heigth - board.height;
        for _ in 0..to_add {
            let mut row = Vec::new();
            for _ in 0..board.width {
                row.push(Tile::Empty);
            }
            board.tiles.push(row);
        }
    } else if new_heigth < board.height {
        let to_del = board.height - new_heigth;
        for _ in 0..to_del {
            board.tiles.pop();
        }
    }

    board.width = new_width;
    board.height = new_heigth;
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
        current_state.set(state).unwrap_or_default();
    }
}

fn spawn_tiles(commands: &mut Commands, windows: &Windows, board: &Board) {
    let rs_params = TileResizeParams::new(windows, board);
    for (y, row) in board.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
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
    tile_size: f32,
    tile_inner_size: Vec2,
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

        Self {
            tile_size,
            tile_inner_size,

            // Think from the middle of the sceen
            board_start_x: (LEFT_BAR_WIDTH_PX - board_width_px) / 2.,
            board_start_y: (board_height_px - TOP_BAR_HEIGHT_PX) / 2.,
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
    if mouse_button_input.pressed(MouseButton::Left) {
        let tile = settile_state_to_tile(set_tile_state.current().clone());
        set_tile(&windows, &mut state.current_map, &mut editor_tiles, tile);
    } else if mouse_button_input.pressed(MouseButton::Right) {
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
                board.tiles[tile.pos.y as usize][tile.pos.x as usize] = tile_to;
                break;
            }
        }
    }
}

fn is_hover(cursor_pos: Vec2, sprite: &Sprite, transform: &Transform) -> bool {
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
