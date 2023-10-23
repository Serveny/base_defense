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

type QueryBuildMenuCircle<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform),
    (With<BuildMenuCircle>, Without<Tower>, Without<Building>),
>;

pub(in crate::game) type QueryTowerMenuParents<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform, &'a Tower),
    (
        With<TowerParent>,
        With<BuildMenuScreen>,
        Without<Building>,
        Without<BuildingBase>,
    ),
>;

pub(in crate::game) type QueryBuildingMenuParents<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Visibility, &'a mut Transform, &'a Building),
    (
        With<BuildingBase>,
        With<BuildMenuScreen>,
        Without<Tower>,
        Without<TowerParent>,
    ),
>;

type QueryBuildMenu<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a mut Visibility), (With<BuildMenuScreen>, Without<TowerRangeCircle>)>;

#[derive(Event, Clone, Copy)]
pub enum BuildMenuScrollEvent {
    Before = -1,
    After = 1,
}

pub(super) fn on_scroll(
    mut evr: EventReader<BuildMenuScrollEvent>,
    mut tbm: ResMut<BuildMenu>,
    mut q_tower: QueryTowerMenuParents,
    mut q_building: QueryBuildingMenuParents,
    board: Res<Board>,
) {
    for ev in evr.iter() {
        if tbm.is_open {
            scroll(
                &mut tbm,
                &mut q_tower,
                &mut q_building,
                &board,
                *ev as isize,
            );
        }
    }
}

#[derive(Event)]
pub struct BuildMenuOpenEvent(pub UVec2);

pub(super) fn on_open(
    mut evr: EventReader<BuildMenuOpenEvent>,
    mut tbm: ResMut<BuildMenu>,
    mut q_circle: QueryBuildMenuCircle,
    mut q_tower: QueryTowerMenuParents,
    mut q_building: QueryBuildingMenuParents,
    board: Res<Board>,
) {
    for ev in evr.iter() {
        let pos = &ev.0;
        if let Some(tile) = board.get_tile(pos) {
            let translation = Vec2Board::from_uvec2_middle(pos).to_scaled_vec3(3.);
            set_build_circle(&mut q_circle, translation);
            show_preview(&mut tbm, &mut q_tower, &mut q_building, translation, tile);
            tbm.is_open = true;
            tbm.is_visible = true;
            tbm.tile_pos = *pos;
        }
    }
}

#[derive(Event)]
pub struct BuildMenuCloseEvent;

pub(super) fn on_close(
    mut evr: EventReader<BuildMenuCloseEvent>,
    mut bm_hide_ev: EventWriter<BuildMenuHideEvent>,
    mut tbm: ResMut<BuildMenu>,
) {
    for _ in evr.iter() {
        bm_hide_ev.send(BuildMenuHideEvent);
        tbm.is_open = false;
    }
}

#[derive(Event)]
pub struct BuildMenuHideEvent;

pub(super) fn on_hide(
    mut evr: EventReader<BuildMenuHideEvent>,
    mut tbm: ResMut<BuildMenu>,
    mut q_tm: QueryBuildMenu,
) {
    for _ in evr.iter() {
        for (_, mut visi) in q_tm.iter_mut() {
            *visi = Visibility::Hidden;
        }
        tbm.is_visible = false;
    }
}

fn set_build_circle(q_tmc: &mut QueryBuildMenuCircle, translation: Vec3) {
    let mut circle = q_tmc.single_mut();
    *circle.0 = Visibility::Visible;
    circle.1.translation = translation;
}

fn show_preview(
    tm: &mut BuildMenu,
    q_tower: &mut QueryTowerMenuParents,
    q_building: &mut QueryBuildingMenuParents,
    translation: Vec3,
    tile: &Tile,
) {
    hide_tower_preview_base(q_tower);
    hide_building_preview_base(q_building);

    match *tile {
        Tile::TowerGround => show_preview_tower(q_tower, translation, tm.selected_tower_index),
        Tile::BuildingGround => {
            show_preview_building(q_building, translation, tm.selected_building_index)
        }
        _ => (),
    }
}

fn show_preview_tower(q_tower: &mut QueryTowerMenuParents, translation: Vec3, selected_i: usize) {
    for (i, (mut visi, mut transform, _)) in q_tower.iter_mut().enumerate() {
        if i == selected_i {
            transform.translation = translation;
            transform.scale = Vec3::new(0.5, 0.5, 1.);
            *visi = Visibility::Visible;
        }
    }
}

fn show_preview_building(
    q_building: &mut QueryBuildingMenuParents,
    translation: Vec3,
    selected_i: usize,
) {
    for (i, (mut visi, mut transform, _)) in q_building.iter_mut().enumerate() {
        if i == selected_i {
            transform.translation = translation;
            transform.scale = Vec3::new(0.5, 0.5, 1.);
            *visi = Visibility::Visible;
        }
    }
}

fn hide_tower_preview_base(q_tower: &mut QueryTowerMenuParents) {
    q_tower.for_each_mut(|(mut visi, _, _)| {
        if *visi == Visibility::Visible {
            *visi = Visibility::Hidden;
        }
    });
}

fn hide_building_preview_base(q_building: &mut QueryBuildingMenuParents) {
    q_building.for_each_mut(|(mut visi, _, _)| {
        if *visi == Visibility::Visible {
            *visi = Visibility::Hidden;
        }
    });
}

fn scroll(
    tm: &mut BuildMenu,
    q_tower: &mut QueryTowerMenuParents,
    q_building: &mut QueryBuildingMenuParents,
    board: &Board,
    additor: isize,
) {
    if let Some(tile) = board.get_tile(&tm.tile_pos) {
        let translation = Vec2Board::from_uvec2_middle(&tm.tile_pos).to_scaled_vec3(3.);
        if *tile == Tile::TowerGround {
            let count = q_tower.iter().count();
            let new_i = tm.selected_tower_index as isize + additor;
            if count > 1 {
                tm.selected_tower_index = new_i.rem_euclid(count as isize) as usize;
                show_preview(tm, q_tower, q_building, translation, tile);
            }
        } else {
            let count = q_building.iter().count();
            let new_i = tm.selected_building_index as isize + additor;
            if count > 1 {
                tm.selected_building_index = new_i.rem_euclid(count as isize) as usize;
                show_preview(tm, q_tower, q_building, translation, tile);
            }
        }
    }
}

#[derive(Event)]
pub struct BuildMenuBuildEvent;

#[allow(clippy::too_many_arguments)]
pub(super) fn on_build(
    mut evr: EventReader<BuildMenuBuildEvent>,
    mut cmds: Commands,
    mut board: ResMut<Board>,
    tbm: ResMut<BuildMenu>,
    mut res_actions: EventWriter<ResourcesEvent>,
    mut bm_close_ev: EventWriter<BuildMenuCloseEvent>,
    q_tower: Query<&Tower>,
    q_qmp_tower: QueryTowerMenuParents,
    q_qmp_building: QueryBuildingMenuParents,
) {
    for _ in evr.iter() {
        if let Some(tile) = board.get_tile_mut(&tbm.tile_pos) {
            if !is_tile_occupied_tower(&q_tower, tbm.tile_pos) {
                match tile {
                    Tile::TowerGround => {
                        place_tower(
                            &mut cmds,
                            &mut res_actions,
                            tbm.get_selected_tower(&q_qmp_tower),
                            &tbm.tile_pos,
                        );
                    }
                    Tile::BuildingGround => {
                        place_building(
                            &mut cmds,
                            &mut res_actions,
                            tbm.get_selected_building(&q_qmp_building),
                            &tbm.tile_pos,
                        );
                    }
                    _ => (),
                };
            }
        }

        bm_close_ev.send(BuildMenuCloseEvent);
    }
}

fn is_tile_occupied_tower(query: &Query<&Tower>, tile_pos: UVec2) -> bool {
    query
        .iter()
        .any(|tower| tower.values().pos.as_uvec2() == tile_pos)
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
