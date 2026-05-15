use crate::{
    balance::{
        FACTORY_ENERGY_COST, FACTORY_MATERIALS_COST, LASER_TOWER_ENERGY_COST,
        LASER_TOWER_MATERIALS_COST, POWER_PLANT_ENERGY_COST, POWER_PLANT_MATERIALS_COST,
        ROCKET_TOWER_ENERGY_COST, ROCKET_TOWER_MATERIALS_COST,
    },
    board::{visualisation::TILE_SIZE, Board, Tile},
    game::{
        build_menus::{
            BuildMenu, BuildMenuCircle, BuildMenuCostPanel, BuildMenuEnergyCostText,
            BuildMenuMaterialsCostText, BuildMenuScreen,
        },
        GameScreen,
    },
    utils::{
        buildings::{
            factory::{spawn_factory, Factory},
            power_plant::{spawn_power_plant, PowerPlant},
            Building, BuildingBase,
        },
        towers::{draw_tower, ChildOfTower, Tower, TowerRangeCircle},
        Energy, Materials, Vec2Board,
    },
};
use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use super::resources::{consume, ResourcesMessage};

const BUILD_COST_TEXT_Y_OFFSET: f32 = -0.58;

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
        With<ChildOfTower>,
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
        Without<ChildOfTower>,
    ),
>;

type QueryBuildMenu<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a mut Visibility), (With<BuildMenuScreen>, Without<TowerRangeCircle>)>;

type QueryBuildMenuCostPanel<'w, 's, 'a> = Query<
    'w,
    's,
    (&'a mut Transform, &'a mut Visibility),
    (
        With<BuildMenuCostPanel>,
        Without<BuildMenuCircle>,
        Without<ChildOfTower>,
        Without<BuildingBase>,
        Without<Tower>,
        Without<Building>,
    ),
>;

type QueryBuildMenuEnergyCostText<'w, 's, 'a> = Query<
    'w,
    's,
    &'a mut Text2d,
    (
        With<BuildMenuEnergyCostText>,
        Without<BuildMenuMaterialsCostText>,
    ),
>;

type QueryBuildMenuMaterialsCostText<'w, 's, 'a> = Query<
    'w,
    's,
    &'a mut Text2d,
    (
        With<BuildMenuMaterialsCostText>,
        Without<BuildMenuEnergyCostText>,
    ),
>;

#[derive(Message, Clone, Copy)]
pub enum BuildMenuScrollMessage {
    Before = -1,
    After = 1,
}

pub(super) fn on_scroll(
    mut evr: MessageReader<BuildMenuScrollMessage>,
    mut tbm: ResMut<BuildMenu>,
    mut q_tower: QueryTowerMenuParents,
    mut q_building: QueryBuildingMenuParents,
    mut q_cost_panel: QueryBuildMenuCostPanel,
    mut q_energy_cost: QueryBuildMenuEnergyCostText,
    mut q_materials_cost: QueryBuildMenuMaterialsCostText,
    board: Res<Board>,
) {
    for ev in evr.read() {
        if tbm.is_open {
            scroll(
                &mut tbm,
                &mut q_tower,
                &mut q_building,
                &mut q_cost_panel,
                &mut q_energy_cost,
                &mut q_materials_cost,
                &board,
                *ev as isize,
            );
        }
    }
}

#[derive(Message)]
pub struct BuildMenuOpenMessage(pub UVec2);

pub(super) fn on_open(
    mut evr: MessageReader<BuildMenuOpenMessage>,
    mut tbm: ResMut<BuildMenu>,
    mut q_circle: QueryBuildMenuCircle,
    mut q_tower: QueryTowerMenuParents,
    mut q_building: QueryBuildingMenuParents,
    mut q_cost_panel: QueryBuildMenuCostPanel,
    mut q_energy_cost: QueryBuildMenuEnergyCostText,
    mut q_materials_cost: QueryBuildMenuMaterialsCostText,
    board: Res<Board>,
) {
    for ev in evr.read() {
        let pos = &ev.0;
        if let Some(tile) = board.get_tile(pos) {
            let translation = Vec2Board::from_uvec2_middle(pos).to_scaled_vec3(3.);
            set_build_circle(&mut q_circle, translation);
            show_preview(
                &mut tbm,
                &mut q_tower,
                &mut q_building,
                &mut q_cost_panel,
                &mut q_energy_cost,
                &mut q_materials_cost,
                translation,
                tile,
            );
            tbm.is_open = true;
            tbm.is_visible = true;
            tbm.tile_pos = *pos;
        }
    }
}

#[derive(Message)]
pub struct BuildMenuCloseMessage;

pub(super) fn on_close(
    mut evr: MessageReader<BuildMenuCloseMessage>,
    mut bm_hide_ev: MessageWriter<BuildMenuHideMessage>,
    mut tbm: ResMut<BuildMenu>,
) {
    for _ in evr.read() {
        bm_hide_ev.write(BuildMenuHideMessage);
        tbm.is_open = false;
    }
}

#[derive(Message)]
pub struct BuildMenuHideMessage;

pub(super) fn on_hide(
    mut evr: MessageReader<BuildMenuHideMessage>,
    mut tbm: ResMut<BuildMenu>,
    mut q_tm: QueryBuildMenu,
) {
    for _ in evr.read() {
        for (_, mut visi) in q_tm.iter_mut() {
            *visi = Visibility::Hidden;
        }
        tbm.is_visible = false;
    }
}

fn set_build_circle(q_tmc: &mut QueryBuildMenuCircle, translation: Vec3) {
    let Ok(mut circle) = q_tmc.single_mut() else {
        return;
    };
    *circle.0 = Visibility::Visible;
    circle.1.translation = translation;
}

fn show_preview(
    tm: &mut BuildMenu,
    q_tower: &mut QueryTowerMenuParents,
    q_building: &mut QueryBuildingMenuParents,
    q_cost_panel: &mut QueryBuildMenuCostPanel,
    q_energy_cost: &mut QueryBuildMenuEnergyCostText,
    q_materials_cost: &mut QueryBuildMenuMaterialsCostText,
    translation: Vec3,
    tile: &Tile,
) {
    hide_tower_preview_base(q_tower);
    hide_building_preview_base(q_building);

    match *tile {
        Tile::TowerGround => show_build_costs(
            q_cost_panel,
            q_energy_cost,
            q_materials_cost,
            translation,
            show_preview_tower(q_tower, translation, tm.selected_tower_index),
        ),
        Tile::BuildingGround => {
            show_build_costs(
                q_cost_panel,
                q_energy_cost,
                q_materials_cost,
                translation,
                show_preview_building(q_building, translation, tm.selected_building_index),
            );
        }
        _ => hide_build_costs(q_cost_panel),
    }
}

fn show_preview_tower(
    q_tower: &mut QueryTowerMenuParents,
    translation: Vec3,
    selected_i: usize,
) -> Option<(Energy, Materials)> {
    for (i, (mut visi, mut transform, tower)) in q_tower.iter_mut().enumerate() {
        if i == selected_i {
            transform.translation = translation;
            transform.scale = Vec3::new(0.5, 0.5, 1.);
            *visi = Visibility::Visible;
            return Some(tower_build_cost(tower));
        }
    }
    None
}

fn show_preview_building(
    q_building: &mut QueryBuildingMenuParents,
    translation: Vec3,
    selected_i: usize,
) -> Option<(Energy, Materials)> {
    for (i, (mut visi, mut transform, building)) in q_building.iter_mut().enumerate() {
        if i == selected_i {
            transform.translation = translation;
            transform.scale = Vec3::new(0.5, 0.5, 1.);
            *visi = Visibility::Visible;
            return Some(building_build_cost(building));
        }
    }
    None
}

fn hide_tower_preview_base(q_tower: &mut QueryTowerMenuParents) {
    q_tower.iter_mut().for_each(|(mut visi, _, _)| {
        if *visi == Visibility::Visible {
            *visi = Visibility::Hidden;
        }
    });
}

fn hide_building_preview_base(q_building: &mut QueryBuildingMenuParents) {
    q_building.iter_mut().for_each(|(mut visi, _, _)| {
        if *visi == Visibility::Visible {
            *visi = Visibility::Hidden;
        }
    });
}

fn scroll(
    tm: &mut BuildMenu,
    q_tower: &mut QueryTowerMenuParents,
    q_building: &mut QueryBuildingMenuParents,
    q_cost_panel: &mut QueryBuildMenuCostPanel,
    q_energy_cost: &mut QueryBuildMenuEnergyCostText,
    q_materials_cost: &mut QueryBuildMenuMaterialsCostText,
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
                show_preview(
                    tm,
                    q_tower,
                    q_building,
                    q_cost_panel,
                    q_energy_cost,
                    q_materials_cost,
                    translation,
                    tile,
                );
            }
        } else {
            let count = q_building.iter().count();
            let new_i = tm.selected_building_index as isize + additor;
            if count > 1 {
                tm.selected_building_index = new_i.rem_euclid(count as isize) as usize;
                show_preview(
                    tm,
                    q_tower,
                    q_building,
                    q_cost_panel,
                    q_energy_cost,
                    q_materials_cost,
                    translation,
                    tile,
                );
            }
        }
    }
}

#[derive(Message)]
pub struct BuildMenuBuildMessage;

#[allow(clippy::too_many_arguments)]
pub(super) fn on_build(
    mut evr: MessageReader<BuildMenuBuildMessage>,
    mut cmds: Commands,
    mut board: ResMut<Board>,
    tbm: ResMut<BuildMenu>,
    mut res_actions: MessageWriter<ResourcesMessage>,
    mut bm_close_ev: MessageWriter<BuildMenuCloseMessage>,
    q_tower: Query<&Tower>,
    q_qmp_tower: QueryTowerMenuParents,
    q_qmp_building: QueryBuildingMenuParents,
) {
    for _ in evr.read() {
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

        bm_close_ev.write(BuildMenuCloseMessage);
    }
}

fn is_tile_occupied_tower(query: &Query<&Tower>, tile_pos: UVec2) -> bool {
    query
        .iter()
        .any(|tower| tower.values().pos.as_uvec2() == tile_pos)
}

fn place_tower(
    cmds: &mut Commands,
    res_actions: &mut MessageWriter<ResourcesMessage>,
    tower: Option<&Tower>,
    pos: &UVec2,
) {
    if let Some(tower) = tower {
        let pos = Vec2Board::from_uvec2_middle(pos);
        draw_tower::<GameScreen>(cmds, pos, tower);
        consume(res_actions, negate_cost(tower_build_cost(tower)), pos);
    }
}

fn place_building(
    cmds: &mut Commands,
    res_actions: &mut MessageWriter<ResourcesMessage>,
    building: Option<&Building>,
    pos: &UVec2,
) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    match building {
        Some(building @ Building::PowerPlant) => {
            spawn_power_plant::<GameScreen>(cmds, PowerPlant::new(pos), TILE_SIZE);
            consume(res_actions, negate_cost(building_build_cost(building)), pos);
        }
        Some(building @ Building::Factory) => {
            spawn_factory::<GameScreen>(cmds, Factory::new(pos), TILE_SIZE);
            consume(res_actions, negate_cost(building_build_cost(building)), pos);
        }
        None => (),
    }
}

fn show_build_costs(
    q_cost_panel: &mut QueryBuildMenuCostPanel,
    q_energy_cost: &mut QueryBuildMenuEnergyCostText,
    q_materials_cost: &mut QueryBuildMenuMaterialsCostText,
    translation: Vec3,
    cost: Option<(Energy, Materials)>,
) {
    let Some((energy_cost, materials_cost)) = cost else {
        hide_build_costs(q_cost_panel);
        return;
    };
    if let Ok((mut transform, mut visibility)) = q_cost_panel.single_mut() {
        transform.translation =
            translation + Vec3::new(0., TILE_SIZE * BUILD_COST_TEXT_Y_OFFSET, 3.);
        *visibility = Visibility::Visible;
    }
    set_build_cost_text(q_energy_cost, energy_cost);
    set_build_cost_text(q_materials_cost, materials_cost);
}

fn set_build_cost_text(q_text: &mut Query<&mut Text2d, impl QueryFilter>, cost: f32) {
    let Ok(mut text) = q_text.single_mut() else {
        return;
    };
    text.0 = format!("-{cost:.0}");
}

fn hide_build_costs(q_cost_panel: &mut QueryBuildMenuCostPanel) {
    if let Ok((_, mut visibility)) = q_cost_panel.single_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn tower_build_cost(tower: &Tower) -> (Energy, Materials) {
    match tower {
        Tower::Laser(_) => (LASER_TOWER_ENERGY_COST, LASER_TOWER_MATERIALS_COST),
        Tower::Microwave(_) => todo!(),
        Tower::Rocket(_) => (ROCKET_TOWER_ENERGY_COST, ROCKET_TOWER_MATERIALS_COST),
        Tower::Grenade(_) => todo!(),
    }
}

fn building_build_cost(building: &Building) -> (Energy, Materials) {
    match building {
        Building::PowerPlant => (POWER_PLANT_ENERGY_COST, POWER_PLANT_MATERIALS_COST),
        Building::Factory => (FACTORY_ENERGY_COST, FACTORY_MATERIALS_COST),
    }
}

fn negate_cost(cost: (Energy, Materials)) -> (Energy, Materials) {
    (-cost.0, -cost.1)
}
