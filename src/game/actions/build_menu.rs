use crate::{
    board::{Board, Tile},
    game::{
        build_menus::{BuildMenu, BuildMenuCircle, BuildMenuScreen},
        GameScreen,
    },
    utils::{
        buildings::{Building, BuildingBase},
        towers::{draw_tower, Tower, TowerBase},
        Vec2Board,
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
}

#[allow(clippy::too_many_arguments)]
pub(in crate::game) fn on_tower_menu_actions(
    mut actions: EventReader<BuildMenuActionsEvent>,
    mut cmds: Commands,
    mut board: ResMut<Board>,
    mut queries: QueriesTowerMenuAction,
    mut tm: ResMut<BuildMenu>,
) {
    use BuildMenuActionsEvent::*;
    if !actions.is_empty() {
        for action in actions.iter() {
            match action {
                Open(pos) => open(&mut tm, &mut queries, pos),
                Close => close(&mut tm, &mut queries.p1()),
                ScollUp => scroll(&mut tm, &mut queries, -1),
                ScollDown => scroll(&mut tm, &mut queries, 1),
                Build => {
                    on_build_tower(&mut cmds, &mut board, &tm, &queries.p2());
                    close(&mut tm, &mut queries.p1());
                }
            }
        }
    }
}

fn open(tbm: &mut BuildMenu, queries: &mut QueriesTowerMenuAction, pos: &UVec2) {
    let translation = Vec2Board::from_uvec2_middle(pos).to_scaled_vec3(3.);
    set_build_circle(&mut queries.p0(), translation);
    show_preview_tower(tbm, queries, translation);
    tbm.tile_pos = *pos;
    tbm.is_open = true;
}

fn close(tbm: &mut BuildMenu, q_tm: &mut QueryBuildMenu) {
    for (_, mut visi) in q_tm.iter_mut() {
        visi.is_visible = false;
    }
    tbm.is_open = false;
}

fn set_build_circle(q_tmc: &mut QueryBuildMenuCircle, translation: Vec3) {
    let mut circle = q_tmc.single_mut();
    circle.0.is_visible = true;
    circle.1.translation = translation;
}

fn show_preview_tower(tm: &mut BuildMenu, queries: &mut QueriesTowerMenuAction, translation: Vec3) {
    if let Some(to_hide) = hide_preview_base(&mut queries.p2()) {
        set_preview_tower_children(&mut queries.p1(), to_hide, false);
    }
    if let Some(to_show) = show_preview_base(tm, &mut queries.p2(), translation) {
        set_preview_tower_children(&mut queries.p1(), to_show, true);
    }
}

fn show_preview_base<TBuild: Component, TBase: Component>(
    tm: &mut BuildMenu,
    q_tm: &mut QueryMenuBases<TBuild, TBase>,
    translation: Vec3,
) -> Option<Children> {
    for (i, (mut visi, mut transform, children, _)) in q_tm.iter_mut().enumerate() {
        if i == tm.selected_tower_index {
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
    for (mut visi, _, children, _) in q_tm.iter_mut() {
        if visi.is_visible {
            visi.is_visible = false;
            return Some(children.clone());
        }
    }
    None
}

fn set_preview_tower_children(q_tms: &mut QueryBuildMenu, children: Children, is_visible: bool) {
    for child in children.iter() {
        if let Ok((_, mut visi)) = q_tms.get_mut(*child) {
            visi.is_visible = is_visible;
        }
    }
}

fn scroll(tm: &mut BuildMenu, queries: &mut QueriesTowerMenuAction, additor: isize) {
    let translation = Vec2Board::from_uvec2_middle(&tm.tile_pos).to_scaled_vec3(3.);
    let count = queries.p2().iter().count();
    let new_i = tm.selected_tower_index as isize + additor as isize;
    if count > 1 {
        tm.selected_tower_index = new_i.rem_euclid(count as isize) as usize;
        show_preview_tower(tm, queries, translation);
    }
}

fn on_build_tower(
    cmds: &mut Commands,
    board: &mut Board,
    tm: &BuildMenu,
    q_tb: &QueryMenuBases<Tower, TowerBase>,
) {
    if let Some(tile) = board.get_tile_mut(&tm.tile_pos) {
        match tile {
            Tile::TowerGround(None) => {
                place_tower(cmds, tm.get_selected(q_tb), &tm.tile_pos);
            }
            Tile::BuildingGround(_) => todo!(),
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

fn change_menu() {}
