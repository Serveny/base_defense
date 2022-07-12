use super::{
    actions::tower_menu::{QueryTowerBase, TowerMenuActionsEvent},
    BaseLevel,
};
use crate::{
    board::visualisation::TILE_SIZE,
    utils::{towers::Tower, Vec2Board},
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

#[derive(Component, Default)]
pub(super) struct TowerMenuScreen;

#[derive(Default)]
pub(super) struct TowerMenu {
    pub is_open: bool,
    pub selected_tower_index: usize,
    pub tile_pos: UVec2,
}

impl TowerMenu {
    fn towers() -> [(BaseLevel, Tower); 2] {
        let pos = Vec2Board::default();
        [(1, Tower::laser(pos)), (1, Tower::rocket(pos))]
    }

    //pub fn new_available_towers(base_lvl: BaseLevel) -> Vec<Tower> {
    //Self::towers()
    //.iter()
    //.filter(|item| item.0 == base_lvl)
    //.map(|item| item.1.clone())
    //.collect()
    //}

    pub fn available_towers(base_lvl: BaseLevel) -> Vec<Tower> {
        Self::towers()
            .iter()
            .filter(|item| item.0 <= base_lvl)
            .map(|item| item.1.clone())
            .collect()
    }

    pub fn get_selected_tower<'a>(&self, q_tower_bases: &'a QueryTowerBase) -> Option<&'a Tower> {
        for (i, (_, _, _, tower)) in q_tower_bases.iter().enumerate() {
            if i == self.selected_tower_index {
                return Some(tower);
            }
        }
        None
    }
}

#[derive(Component)]
pub struct TowerMenuCircle;

pub(super) fn draw_tower_build_menu(
    cmds: &mut Commands,
    mut actions: EventWriter<TowerMenuActionsEvent>,
    base_lvl: BaseLevel,
) {
    cmds.spawn_bundle(menu_circle_shape(TILE_SIZE))
        .insert(TowerMenuCircle)
        .insert(TowerMenuScreen);

    let mut towers = TowerMenu::available_towers(base_lvl);
    while let Some(tower) = towers.pop() {
        // println!("{:?}", tower);
        tower.draw_default::<TowerMenuScreen>(cmds);
    }
    actions.send(TowerMenuActionsEvent::Close);
}

fn menu_circle_shape(tile_size: f32) -> ShapeBundle {
    let shape = Circle {
        center: Vec2::default(),
        radius: tile_size / 2.5,
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::rgba(0.75, 0.75, 0.75, 0.5)),
            outline_mode: StrokeMode::new(Color::rgba(0.25, 0.25, 0.25, 0.5), tile_size / 32.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.2),
            ..Default::default()
        },
    )
}
