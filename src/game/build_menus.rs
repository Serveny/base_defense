use crate::utils::{towers::Tower, Vec2Board};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*, shapes::Circle};

use super::{actions::build_menu::QueryMenuBases, BaseLevel};

pub mod building;
pub mod tower;

#[derive(Component, Default)]
pub(super) struct BuildMenuScreen;

#[derive(Default)]
pub(super) struct BuildMenu {
    pub is_open: bool,
    pub selected_tower_index: usize,
    pub selected_building_index: usize,
    pub tile_pos: UVec2,
}

impl BuildMenu {
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

    pub fn get_selected<'a, TBuild: Component, TBase: Component>(
        &self,
        q_tower_bases: &'a QueryMenuBases<TBuild, TBase>,
    ) -> Option<&'a TBuild> {
        for (i, (_, _, _, build)) in q_tower_bases.iter().enumerate() {
            if i == self.selected_tower_index {
                return Some(build);
            }
        }
        None
    }
}

#[derive(Component)]
pub struct BuildMenuCircle;

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
