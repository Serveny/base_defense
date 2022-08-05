use crate::{
    board::{visualisation::TILE_SIZE, Board, Tile},
    game::{
        build_menus::{BuildMenu, BuildMenuCircle, BuildMenuScreen},
        GameScreen,
    },
    utils::{
        buildings::{
            factory::{spawn_factory, Factory},
            power_plant::{spawn_power_plant, PowerPlant},
            Building, BuildingBase,
        },
        towers::{draw_tower, Tower, TowerParent, TowerRangeCircle},
        Vec2Board,
    },
};
use bevy::prelude::*;

use super::resources::{consume, ResourcesEvent};

type QueriesTowerMenuAction<'w, 's, 'a> = ParamSet<
    'w,
    's,
    (
        QueryBuildMenuCircle<'w, 's, 'a>,
        QueryBuildMenu<'w, 's, 'a>,
        QueryMenuParents<'w, 's, 'a, Tower, TowerParent>,
        QueryMenuParents<'w, 's, 'a, Building, BuildingBase>,
    ),
>;

type QueryBuildMenuCircle<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a mut Transform), With<BuildMenuCircle>>;

pub(in crate::game) type QueryMenuParents<'w, 's, 'a, TBuild, TBase> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform, &'a TBuild),
    (With<TBase>, With<BuildMenuScreen>),
>;

type QueryBuildMenu<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a mut Visibility), (With<BuildMenuScreen>, Without<TowerRangeCircle>)>;

pub enum BuildMenuActionsEvent {
    Open(UVec2),
    ScollUp,
    ScollDown,
    Close,
    Build,
    Hide,
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn on_tower_menu_actions(
    mut actions: EventReader<BuildMenuActionsEvent>,
    mut cmds: Commands,
    mut board: ResMut<Board>,
    mut queries: QueriesTowerMenuAction,
    mut tbm: ResMut<BuildMenu>,
    mut res_actions: EventWriter<ResourcesEvent>,
) {
    use BuildMenuActionsEvent::*;
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                Open(pos) => open(&mut tbm, &mut queries, &board, pos),
                Close => close(&mut tbm, &mut queries.p1()),
                ScollUp => scroll(&mut tbm, &mut queries, &board, -1),
                ScollDown => scroll(&mut tbm, &mut queries, &board, 1),
                Build => {
                    on_build(&mut cmds, &mut board, &tbm, &mut queries, &mut res_actions);
                    close(&mut tbm, &mut queries.p1());
                }
                Hide => hide(&mut tbm, &mut queries.p1()),
            }
        }
    }
}

fn open(tbm: &mut BuildMenu, queries: &mut QueriesTowerMenuAction, board: &Board, pos: &UVec2) {
    if let Some(tile) = board.get_tile(pos) {
        let translation = Vec2Board::from_uvec2_middle(pos).to_scaled_vec3(3.);
        set_build_circle(&mut queries.p0(), translation);
        show_preview(tbm, queries, translation, tile);
        tbm.tile_pos = *pos;
        tbm.is_open = true;
        tbm.is_visible = true;
    }
}

fn close(tbm: &mut BuildMenu, q_tm: &mut QueryBuildMenu) {
    hide(tbm, q_tm);
    tbm.is_open = false;
}

fn hide(tbm: &mut BuildMenu, q_tm: &mut QueryBuildMenu) {
    for (_, mut visi) in q_tm.iter_mut() {
        visi.is_visible = false;
    }
    tbm.is_visible = false;
}

fn set_build_circle(q_tmc: &mut QueryBuildMenuCircle, translation: Vec3) {
    let mut circle = q_tmc.single_mut();
    circle.0.is_visible = true;
    circle.1.translation = translation;
}

fn show_preview(
    tm: &mut BuildMenu,
    queries: &mut QueriesTowerMenuAction,
    translation: Vec3,
    tile: &Tile,
) {
    hide_preview_base(&mut queries.p2());
    hide_preview_base(&mut queries.p3());

    match *tile {
        Tile::TowerGround => {
            show_preview_tower(&mut queries.p2(), translation, tm.selected_tower_index)
        }
        _ => show_preview_tower(&mut queries.p3(), translation, tm.selected_building_index),
    }
}

fn show_preview_tower<TBuild: Component, TBase: Component>(
    q_tm: &mut QueryMenuParents<TBuild, TBase>,
    translation: Vec3,
    selected_i: usize,
) {
    for (i, (mut visi, mut transform, _)) in q_tm.iter_mut().enumerate() {
        if i == selected_i {
            transform.translation = translation;
            transform.scale = Vec3::new(0.5, 0.5, 1.);
            visi.is_visible = true;
        }
    }
}

fn hide_preview_base<TBuild: Component, TBase: Component>(
    q_tm: &mut QueryMenuParents<TBuild, TBase>,
) {
    q_tm.for_each_mut(|(mut visi, _, _)| {
        if visi.is_visible {
            visi.is_visible = false;
        }
    });
}

fn scroll(tm: &mut BuildMenu, queries: &mut QueriesTowerMenuAction, board: &Board, additor: isize) {
    if let Some(tile) = board.get_tile(&tm.tile_pos) {
        let translation = Vec2Board::from_uvec2_middle(&tm.tile_pos).to_scaled_vec3(3.);
        if *tile == Tile::TowerGround {
            let count = queries.p2().iter().count();
            let new_i = tm.selected_tower_index as isize + additor as isize;
            if count > 1 {
                tm.selected_tower_index = new_i.rem_euclid(count as isize) as usize;
                show_preview(tm, queries, translation, tile);
            }
        } else {
            let count = queries.p3().iter().count();
            let new_i = tm.selected_building_index as isize + additor as isize;
            if count > 1 {
                tm.selected_building_index = new_i.rem_euclid(count as isize) as usize;
                show_preview(tm, queries, translation, tile);
            }
        }
    }
}

fn on_build(
    cmds: &mut Commands,
    board: &mut Board,
    tm: &BuildMenu,
    queries: &mut QueriesTowerMenuAction,
    res_actions: &mut EventWriter<ResourcesEvent>,
) {
    if let Some(tile) = board.get_tile_mut(&tm.tile_pos) {
        match tile {
            Tile::TowerGround => {
                place_tower(
                    cmds,
                    res_actions,
                    tm.get_selected_tower(&queries.p2()),
                    &tm.tile_pos,
                );
            }
            Tile::BuildingGround => {
                place_building(
                    cmds,
                    res_actions,
                    tm.get_selected_building(&queries.p3()),
                    &tm.tile_pos,
                );
            }
            _ => (),
        }
    }
}

fn place_tower(
    cmds: &mut Commands,
    res_actions: &mut EventWriter<ResourcesEvent>,
    tower: Option<&Tower>,
    pos: &UVec2,
) {
    if let Some(tower) = tower {
        let pos = Vec2Board::from_uvec2_middle(pos);
        draw_tower::<GameScreen>(cmds, pos, tower);
        consume(
            res_actions,
            match *tower {
                Tower::Laser(_) => (-100., -200.),
                Tower::Microwave(_) => todo!(),
                Tower::Rocket(_) => (-1000., -1000.),
                Tower::Grenade(_) => todo!(),
            },
            pos,
        );
    }
}

fn place_building(
    cmds: &mut Commands,
    res_actions: &mut EventWriter<ResourcesEvent>,
    building: Option<&Building>,
    pos: &UVec2,
) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    match building {
        Some(Building::PowerPlant) => {
            spawn_power_plant::<GameScreen>(cmds, PowerPlant::new(pos), TILE_SIZE);
            consume(res_actions, (-500., -600.), pos);
        }
        Some(Building::Factory) => {
            spawn_factory::<GameScreen>(cmds, Factory::new(pos), TILE_SIZE);
            consume(res_actions, (-10000., -1000.), pos);
        }
        None => (),
    }
}
