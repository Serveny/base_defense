use super::{
    enemies::{spawn_enemy_component, Enemy},
    wave::{Wave, WaveState},
    BoardVisu, Game, GameScreen,
};
use crate::{
    board::{
        visualisation::{BoardHoverCross, BoardScreen},
        Board, BoardCache, Tile,
    },
    utils::{
        towers::{delete_range_circle, draw_tower, set_range_cycle, Tower, TowerRangeHoverCircle},
        GameState, Vec2Board,
    },
};
use bevy::prelude::*;
use std::time::Duration;

pub enum GameActionEvent {
    Resize,
    HoverTile(Vec2Board, Tile),
    UnhoverTile,
    BackToMainMenu,
    StartWave,
    EndWave,
    TileLeftClick(UVec2),
}

#[allow(dead_code)]
struct GameActionParams<'w, 's, 'g, 'gs, 'visu, 'b, 'bc, 'ws, 'win, 't> {
    cmds: Commands<'w, 's>,
    game: &'g mut Game,
    game_state: &'gs mut State<GameState>,
    board_visu: &'visu mut BoardVisu,
    board: &'b mut Board,
    board_cache: &'bc BoardCache,
    wave_state: &'ws mut State<WaveState>,
    windows: &'win Windows,
    time: &'t Time,
}

pub(super) fn game_actions(
    cmds: Commands,
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    mut visu: ResMut<BoardVisu>,
    mut wave_state: ResMut<State<WaveState>>,
    mut queries: ParamSet<(
        Query<Entity, With<BoardScreen>>,
        Query<(&Enemy, Entity), With<Enemy>>,
        Query<(Entity, &mut Transform), With<BoardHoverCross>>,
        Query<(Entity, &mut Transform, &mut TowerRangeHoverCircle), With<TowerRangeHoverCircle>>,
        Query<Entity, With<GameScreen>>,
    )>,
    mut game_actions: EventReader<GameActionEvent>,
    mut board: ResMut<Board>,
    board_cache: Res<BoardCache>,
    windows: Res<Windows>,
    time: Res<Time>,
) {
    if !game_actions.is_empty() {
        let mut ga_params = GameActionParams {
            cmds: cmds,
            game: &mut game,
            game_state: &mut game_state,
            board_visu: &mut visu,
            board: &mut board,
            board_cache: &board_cache,
            wave_state: &mut wave_state,
            windows: &windows,
            time: &time,
        };
        for event in game_actions.iter() {
            match event {
                GameActionEvent::Resize => repaint(&mut ga_params, &mut queries),
                GameActionEvent::HoverTile(pos, tile) => {
                    draw_hover_cross(&mut ga_params, &mut queries.p2(), pos, tile);
                    set_range_cycle(
                        &mut ga_params.cmds,
                        ga_params.board_visu,
                        pos,
                        tile,
                        queries.p3(),
                    );
                }
                GameActionEvent::UnhoverTile => {
                    BoardVisu::delete_hover_cross(&mut ga_params.cmds, &mut queries.p2());
                    delete_range_circle(&mut ga_params.cmds, queries.p3());
                }
                GameActionEvent::BackToMainMenu => back_to_main_menu(&mut ga_params, queries.p4()),
                GameActionEvent::StartWave => start_wave(&mut ga_params),
                GameActionEvent::EndWave => end_wave_and_prepare_next(&mut ga_params),
                GameActionEvent::TileLeftClick(pos) => on_tile_click(&mut ga_params, pos),
            }
        }
    }
}

fn repaint(
    ga_params: &mut GameActionParams,
    queries: &mut ParamSet<(
        Query<Entity, With<BoardScreen>>,
        Query<(&Enemy, Entity), With<Enemy>>,
        Query<(Entity, &mut Transform), With<BoardHoverCross>>,
        Query<(Entity, &mut Transform, &mut TowerRangeHoverCircle), With<TowerRangeHoverCircle>>,
        Query<Entity, With<GameScreen>>,
    )>,
) {
    *ga_params.board_visu = create_visu(ga_params.windows, ga_params.board);
    ga_params.board_visu.repaint(
        &mut ga_params.cmds,
        queries.p0().into(),
        ga_params.board,
        ga_params.board_cache,
    );
    resize_enemies(ga_params, queries.p1());
}

fn create_visu(windows: &Windows, board: &Board) -> BoardVisu {
    BoardVisu::new(
        windows.get_primary().unwrap(),
        &board,
        0.,
        0.,
        0.,
        GameScreen,
    )
}

fn draw_hover_cross(
    ga_params: &mut GameActionParams,
    query: &mut Query<(Entity, &mut Transform), With<BoardHoverCross>>,
    pos: &Vec2Board,
    tile: &Tile,
) {
    match tile {
        Tile::TowerGround(_) => {
            ga_params
                .board_visu
                .draw_hover_cross(&mut ga_params.cmds, query, pos)
        }
        Tile::BuildingGround(_) => {
            ga_params
                .board_visu
                .draw_hover_cross(&mut ga_params.cmds, query, pos)
        }
        _ => BoardVisu::delete_hover_cross(&mut ga_params.cmds, query),
    }
}

fn back_to_main_menu(ga_params: &mut GameActionParams, query: Query<Entity, With<GameScreen>>) {
    for entity in query.iter() {
        ga_params.cmds.entity(entity).despawn_recursive();
    }
    ga_params.wave_state.set(WaveState::None).unwrap();
    ga_params.game_state.set(GameState::Menu).unwrap();
}

fn resize_enemies(ga_params: &mut GameActionParams, query: Query<(&Enemy, Entity), With<Enemy>>) {
    let mut enemies = Vec::<Enemy>::new();
    query.for_each(|(enemy, entity)| {
        enemies.push(enemy.clone());
        ga_params.cmds.entity(entity).despawn_recursive();
    });
    for enemy in enemies {
        spawn_enemy_component(&mut ga_params.cmds, ga_params.board_visu, enemy);
    }
}

fn start_wave(ga_params: &mut GameActionParams) {
    ga_params.game.next_wave_spawn = None;
    ga_params.game.wave_no += 1;
    ga_params.cmds.insert_resource(Wave::new(
        ga_params.game.wave_no,
        ga_params.time.last_update().unwrap(),
    ));
    ga_params.wave_state.set(WaveState::WaveRunning).unwrap();
}

fn end_wave_and_prepare_next(ga_params: &mut GameActionParams) {
    ga_params.wave_state.set(WaveState::None).unwrap();
    ga_params.game.next_wave_spawn =
        Some(ga_params.time.last_update().unwrap() + Duration::from_secs(1));
}

fn on_tile_click(ga_params: &mut GameActionParams, pos: &UVec2) {
    if let Some(tile) = ga_params.board.get_tile_mut(pos) {
        match tile {
            Tile::TowerGround(tower) => {
                if tower.is_none() {
                    place_tower(&mut ga_params.cmds, ga_params.board_visu, pos, tower)
                }
            }
            Tile::BuildingGround(_) => todo!(),
            Tile::Road => todo!(),
            Tile::Empty => todo!(),
        }
    }
}

fn place_tower(
    cmds: &mut Commands,
    board_visu: &BoardVisu,
    pos: &UVec2,
    tower: &mut Option<Tower>,
) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    let new_tower = Tower::laser_shot(pos);
    draw_tower(cmds, board_visu, pos, &new_tower);
    *tower = Some(new_tower);
}
