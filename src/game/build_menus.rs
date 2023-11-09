use super::{
    actions::build_menu::{BuildMenuCloseEvent, QueryBuildingMenuParents, QueryTowerMenuParents},
    BaseLevel,
};
use crate::{
    board::visualisation::TILE_SIZE,
    utils::{
        buildings::{
            factory::{spawn_factory, Factory},
            power_plant::{spawn_power_plant, PowerPlant},
            Building,
        },
        towers::Tower,
        Vec2Board,
    },
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

#[derive(Component, Default)]
pub(super) struct BuildMenuScreen;

#[derive(Resource, Default)]
pub(super) struct BuildMenu {
    pub is_open: bool,
    pub is_visible: bool,
    pub selected_tower_index: usize,
    pub selected_building_index: usize,
    pub tile_pos: UVec2,
}

impl BuildMenu {
    pub fn should_open(&self, tile_pos: UVec2) -> bool {
        !self.is_visible || self.tile_pos != tile_pos
    }
    fn towers() -> [(BaseLevel, Tower); 2] {
        let pos = Vec2Board::default();
        [(1, Tower::laser(pos)), (1, Tower::rocket(pos))]
    }

    fn buidings() -> [(BaseLevel, Building); 2] {
        [(1, Building::PowerPlant), (1, Building::Factory)]
    }

    //pub fn new_available_towers(base_lvl: BaseLevel) -> Vec<Tower> {
    //Self::towers()
    //.read()
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

    pub fn available_buildings(base_lvl: BaseLevel) -> Vec<Building> {
        Self::buidings()
            .iter()
            .filter(|item| item.0 <= base_lvl)
            .map(|item| item.1)
            .collect()
    }

    pub fn get_selected_tower<'a>(&self, q_bases: &'a QueryTowerMenuParents) -> Option<&'a Tower> {
        for (i, (_, _, tower)) in q_bases.iter().enumerate() {
            if i == self.selected_tower_index {
                return Some(tower);
            }
        }
        None
    }

    pub fn get_selected_building<'a>(
        &self,
        q_bases: &'a QueryBuildingMenuParents,
    ) -> Option<&'a Building> {
        for (i, (_, _, building)) in q_bases.iter().enumerate() {
            if i == self.selected_building_index {
                return Some(building);
            }
        }
        None
    }
}

#[derive(Component)]
pub struct BuildMenuCircle;

fn menu_circle_shape(tile_size: f32) -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&Circle {
                center: Vec2::default(),
                radius: tile_size / 2.5,
            }),
            spatial: SpatialBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 3.)),
                ..default()
            },
            ..default()
        },
        Fill::color(Color::rgba(0.75, 0.75, 0.75, 0.)),
        Stroke::new(Color::rgba(0.25, 0.25, 0.25, 0.5), tile_size / 32.),
    )
}

pub fn draw_build_menu(
    cmds: &mut Commands,
    mut bm_close_ev: EventWriter<BuildMenuCloseEvent>,
    base_lvl: BaseLevel,
) {
    cmds.spawn(menu_circle_shape(TILE_SIZE))
        .insert(BuildMenuCircle)
        .insert(BuildMenuScreen);

    let mut towers = BuildMenu::available_towers(base_lvl);
    while let Some(tower) = towers.pop() {
        tower.draw_preview::<BuildMenuScreen>(cmds);
    }
    let mut buildings = BuildMenu::available_buildings(base_lvl);
    while let Some(building) = buildings.pop() {
        match building {
            Building::PowerPlant => {
                spawn_power_plant::<BuildMenuScreen>(cmds, PowerPlant::default(), TILE_SIZE)
            }
            Building::Factory => {
                spawn_factory::<BuildMenuScreen>(cmds, Factory::default(), TILE_SIZE)
            }
        }
    }
    bm_close_ev.send(BuildMenuCloseEvent);
}
