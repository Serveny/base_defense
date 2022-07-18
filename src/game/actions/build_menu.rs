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
        towers::{draw_tower, Tower, TowerBase},
        IngameTime, IngameTimestamp, Vec2Board,
    },
};
use bevy::prelude::*;

type QueriesTowerMenuAction<'w, 's, 'a> = ParamSet<
    'w,
    's,
    (
        QueryBuildMenuCircle<'w, 's, 'a>,
        QueryBuildMenu<'w, 's, 'a>,
        QueryMenuBases<'w, 's, 'a, Tower, TowerBase>,
        QueryMenuBases<'w, 's, 'a, Building, BuildingBase>,
    ),
>;

type QueryBuildMenuCircle<'w, 's, 'a> =
    Query<'w, 's, (&'a mut Visibility, &'a mut Transform), With<BuildMenuCircle>>;

pub(in crate::game) type QueryMenuBases<'w, 's, 'a, TBuild, TBase> = Query<
    'w,
    's,
    (
        &'a mut Visibility,
        &'a mut Transform,
        &'a Children,
        &'a TBuild,
    ),
    (With<TBase>, With<BuildMenuScreen>),
>;

type QueryBuildMenu<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a mut Visibility), With<BuildMenuScreen>>;

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
    mut tm: ResMut<BuildMenu>,
    time: Res<IngameTime>,
) {
    use BuildMenuActionsEvent::*;
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                Open(pos) => open(&mut tm, &mut queries, &board, pos),
                Close => close(&mut tm, &mut queries.p1()),
                ScollUp => scroll(&mut tm, &mut queries, &board, -1),
                ScollDown => scroll(&mut tm, &mut queries, &board, 1),
                Build => {
                    on_build(&mut cmds, &mut board, &tm, &mut queries, time.now());
                    close(&mut tm, &mut queries.p1());
                }
                Hide => hide(&mut queries.p1()),
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
    }
}

fn close(tbm: &mut BuildMenu, q_tm: &mut QueryBuildMenu) {
    hide(q_tm);
    tbm.is_open = false;
}

fn hide(q_tm: &mut QueryBuildMenu) {
    for (_, mut visi) in q_tm.iter_mut() {
        visi.is_visible = false;
    }
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
    if let Some(tower_to_hide) = hide_preview_base(&mut queries.p2()) {
        set_preview_children(&mut queries.p1(), tower_to_hide, false);
    } else if let Some(building_to_hide) = hide_preview_base(&mut queries.p3()) {
        set_preview_children(&mut queries.p1(), building_to_hide, false);
    }

    if *tile == Tile::TowerGround {
        if let Some(to_show) =
            show_preview_base(&mut queries.p2(), translation, tm.selected_tower_index)
        {
            set_preview_children(&mut queries.p1(), to_show, true);
        }
    } else if let Some(to_show) =
        show_preview_base(&mut queries.p3(), translation, tm.selected_building_index)
    {
        set_preview_children(&mut queries.p1(), to_show, true);
    };
}

fn show_preview_base<TBuild: Component, TBase: Component>(
    q_tm: &mut QueryMenuBases<TBuild, TBase>,
    translation: Vec3,
    selected_i: usize,
) -> Option<Children> {
    for (i, (mut visi, mut transform, children, _)) in q_tm.iter_mut().enumerate() {
        if i == selected_i {
            transform.translation = translation;
            transform.scale = Vec3::new(0.5, 0.5, 1.);
            visi.is_visible = true;
            return Some(children.clone());
        }
    }
    None
}

fn hide_preview_base<TBuild: Component, TBase: Component>(
    q_tm: &mut QueryMenuBases<TBuild, TBase>,
) -> Option<Children> {
    q_tm.iter_mut().find_map(|(mut visi, _, children, _)| {
        if visi.is_visible {
            visi.is_visible = false;
            return Some(children.clone());
        }
        None
    })
}

fn set_preview_children(q_tms: &mut QueryBuildMenu, children: Children, is_visible: bool) {
    for child in children.iter() {
        if let Ok((_, mut visi)) = q_tms.get_mut(*child) {
            visi.is_visible = is_visible;
        }
    }
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
    now: IngameTimestamp,
) {
    if let Some(tile) = board.get_tile_mut(&tm.tile_pos) {
        match tile {
            Tile::TowerGround => {
                place_tower(cmds, tm.get_selected_tower(&queries.p2()), &tm.tile_pos);
            }
            Tile::BuildingGround => {
                place_building(
                    cmds,
                    tm.get_selected_building(&queries.p3()),
                    &tm.tile_pos,
                    now,
                );
            }
            _ => (),
        }
    }
}

fn place_tower(cmds: &mut Commands, tower: Option<&Tower>, pos: &UVec2) {
    if let Some(tower) = tower {
        let pos = Vec2Board::from_uvec2_middle(pos);
        draw_tower::<GameScreen>(cmds, pos, tower);
    }
}

fn place_building(
    cmds: &mut Commands,
    building: Option<&Building>,
    pos: &UVec2,
    now: IngameTimestamp,
) {
    let pos = Vec2Board::from_uvec2_middle(pos);
    match building {
        Some(Building::PowerPlant) => {
            spawn_power_plant::<GameScreen>(cmds, PowerPlant::new(now, pos), TILE_SIZE)
        }
        Some(Building::Factory) => {
            spawn_factory::<GameScreen>(cmds, Factory::new(now, pos), TILE_SIZE)
        }
        None => (),
    }
}
