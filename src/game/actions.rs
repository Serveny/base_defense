use super::{
    enemies::Enemy,
    wave::{Wave, WaveState},
    BoardVisu, Game, GameScreen, IngameTime,
};
use crate::{
    board::{
        visualisation::{BoardScreen, HoverCrossQuery},
        Board, BoardCache, Tile,
    },
    utils::{
        shots::{laser_shape, Shot},
        towers::{draw_tower, Tower, TowerRangeCircle},
        GameState, IngameTimestamp, Vec2Board,
    },
};
use bevy::prelude::*;
use std::time::Duration;

type GameActionQueries<'w, 's, 'a> = ParamSet<
    'w,
    's,
    (
        Query<'w, 's, Entity, With<BoardScreen>>,
        Query<'w, 's, (Entity, &'a Enemy), With<Enemy>>,
        HoverCrossQuery<'w, 's, 'a>,
        Query<'w, 's, (&'a mut Visibility, &'a TowerRangeCircle), With<TowerRangeCircle>>,
        Query<'w, 's, Entity, With<GameScreen>>,
    ),
>;

pub enum GameActionEvent {
    HoverTile(Vec2Board, Tile),
    UnhoverTile,
    BackToMainMenu,
    StartWave,
    EndWave,
    TileLeftClick(UVec2),
    ActivateOverview,
    DeactivateOverview,
    Shoot(Shot),
}

#[allow(dead_code)]
struct GameActionParams<'w, 's, 'g, 'gs, 'visu, 'b, 'bc, 'ws, 'win> {
    cmds: Commands<'w, 's>,
    game: &'g mut Game,
    game_state: &'gs mut State<GameState>,
    board_visu: &'visu mut BoardVisu,
    board: &'b mut Board,
    board_cache: &'bc BoardCache,
    wave_state: &'ws mut State<WaveState>,
    windows: &'win Windows,
    now: IngameTimestamp,
}

pub(super) fn game_actions(
    cmds: Commands,
    mut game: ResMut<Game>,
    mut game_state: ResMut<State<GameState>>,
    mut visu: ResMut<BoardVisu>,
    mut wave_state: ResMut<State<WaveState>>,
    mut queries: GameActionQueries,
    mut game_actions: EventReader<GameActionEvent>,
    mut board: ResMut<Board>,
    board_cache: Res<BoardCache>,
    windows: Res<Windows>,
    time: Res<IngameTime>,
) {
    if !game_actions.is_empty() {
        let mut ga_params = GameActionParams {
            cmds,
            game: &mut game,
            game_state: &mut game_state,
            board_visu: &mut visu,
            board: &mut board,
            board_cache: &board_cache,
            wave_state: &mut wave_state,
            windows: &windows,
            now: time.elapsed_secs().into(),
        };
        for event in game_actions.iter() {
            match event {
                GameActionEvent::HoverTile(pos, tile) => {
                    draw_hover_cross(&mut ga_params, &mut queries.p2(), pos, tile);
                    if !ga_params.game.is_overview {
                        show_range_circle(&mut queries, &pos.as_uvec2());
                    }
                }
                GameActionEvent::UnhoverTile => {
                    BoardVisu::hide_hover_cross(&mut queries.p2());
                    if !ga_params.game.is_overview {
                        set_range_circles(&mut queries, false);
                    }
                }
                GameActionEvent::BackToMainMenu => back_to_main_menu(&mut ga_params, queries.p4()),
                GameActionEvent::StartWave => start_wave(&mut ga_params),
                GameActionEvent::EndWave => end_wave_and_prepare_next(&mut ga_params),
                GameActionEvent::TileLeftClick(pos) => handle_click(&mut ga_params, pos),
                GameActionEvent::ActivateOverview => {
                    ga_params.game.is_overview = true;
                    set_range_circles(&mut queries, true);
                }
                GameActionEvent::DeactivateOverview => {
                    ga_params.game.is_overview = false;
                    set_range_circles(&mut queries, false);
                }
                GameActionEvent::Shoot(shot) => shoot(&mut ga_params, shot),
            }
        }
    }
}

fn draw_hover_cross(
    ga_params: &mut GameActionParams,
    query: &mut HoverCrossQuery,
    pos: &Vec2Board,
    tile: &Tile,
) {
    match tile {
        Tile::TowerGround(_) => ga_params.board_visu.show_hover_cross(query, pos),
        Tile::BuildingGround(_) => ga_params.board_visu.show_hover_cross(query, pos),
        _ => BoardVisu::hide_hover_cross(query),
    }
}

fn back_to_main_menu(ga_params: &mut GameActionParams, query: Query<Entity, With<GameScreen>>) {
    for entity in query.iter() {
        ga_params.cmds.entity(entity).despawn_recursive();
    }
    ga_params
        .wave_state
        .set(WaveState::None)
        .unwrap_or_default();
    ga_params.game_state.set(GameState::Menu).unwrap();
}

fn start_wave(ga_params: &mut GameActionParams) {
    ga_params.game.next_wave_spawn = None;
    ga_params.game.wave_no += 1;
    ga_params
        .cmds
        .insert_resource(Wave::new(ga_params.game.wave_no, ga_params.now));
    ga_params.wave_state.set(WaveState::WaveRunning).unwrap();
}

fn end_wave_and_prepare_next(ga_params: &mut GameActionParams) {
    ga_params.wave_state.set(WaveState::None).unwrap();
    ga_params.game.next_wave_spawn = Some(ga_params.now + Duration::from_secs(1));
}

fn handle_click(ga_params: &mut GameActionParams, pos: &UVec2) {
    if let Some(tile) = ga_params.board.get_tile_mut(pos) {
        on_tile_click(
            &mut ga_params.cmds,
            ga_params.board_visu,
            ga_params.now,
            tile,
            pos,
        );
    }
}

fn on_tile_click(
    cmds: &mut Commands,
    board_visu: &BoardVisu,
    now: IngameTimestamp,
    tile: &mut Tile,
    pos: &UVec2,
) {
    match tile {
        Tile::TowerGround(tower) => {
            if tower.is_none() {
                place_tower(cmds, tower, board_visu, pos, now);
            }
        }
        Tile::BuildingGround(_) => todo!(),
        _ => (),
    }
}

fn place_tower(
    cmds: &mut Commands,
    tower: &mut Option<Tower>,
    board_visu: &BoardVisu,
    pos: &UVec2,
    now: IngameTimestamp,
) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    let new_tower = Tower::laser_shot(pos, now);
    draw_tower(cmds, board_visu, pos, &new_tower, now);
    *tower = Some(new_tower);
}

fn set_range_circles(queries: &mut GameActionQueries, is_visible: bool) {
    queries
        .p3()
        .for_each_mut(|(mut visi, _)| visi.is_visible = is_visible);
}

fn show_range_circle(queries: &mut GameActionQueries, pos: &UVec2) {
    queries
        .p3()
        .for_each_mut(|(mut visi, comp)| visi.is_visible = **comp == *pos);
}

fn shoot(ga_params: &mut GameActionParams, shot: &Shot) {
    ga_params
        .cmds
        .spawn_bundle(laser_shape(ga_params.board_visu.inner_tile_size))
        .insert(shot.clone());
    println!("Shoot: {:?}", shot);
}
