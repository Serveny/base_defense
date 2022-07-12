use std::time::Duration;

use super::{
    tower_base_shape, tower_circle_shape, tower_range_circle_shape, Tower, TowerBase, TowerCannon,
    TowerRangeCircle, TowerValues,
};
use crate::{
    board::visualisation::TILE_SIZE,
    utils::{
        shots::{Shot, TowerStatus},
        Vec2Board,
    },
};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

impl Tower {
    pub fn laser(pos: Vec2Board) -> Self {
        Self::Laser(TowerValues::laser(pos))
    }
}
impl TowerValues {
    pub fn laser(pos: Vec2Board) -> Self {
        use super::super::shots::laser;
        Self {
            pos,
            range_radius: laser::INIT_RANGE_RADIUS,
            shot: Shot::laser_vals(pos),
            reload_duration: Duration::from_secs(1),
            shoot_duration: Duration::from_secs_f32(laser::INIT_SHOT_DURATION_SECS),

            target_lock: None,
            tower_status: TowerStatus::Waiting,
        }
    }
}

pub(super) fn spawn_laser_tower<TScreen: Component + Default>(
    cmds: &mut Commands,
    vals: TowerValues,
) {
    let color = Color::RED;
    cmds.spawn_bundle(tower_base_shape(vals.pos.to_scaled_vec3(1.), color))
        .with_children(|parent| laser_tower_children::<TScreen>(parent, &vals, color))
        .insert(TowerBase)
        .insert(Tower::Laser(vals))
        .insert(TScreen::default());
}

fn laser_tower_children<TScreen: Component + Default>(
    parent: &mut ChildBuilder,
    vals: &TowerValues,
    color: Color,
) {
    // Tower circle
    parent
        .spawn_bundle(tower_circle_shape())
        .insert(TScreen::default());

    // Tower cannon
    parent
        .spawn_bundle(tower_laser_cannon())
        .insert(TowerCannon)
        .insert(TScreen::default());

    // Range circle
    let mut range_circle = tower_range_circle_shape(vals.range_radius, color);
    range_circle.visibility.is_visible = false;
    parent
        .spawn_bundle(range_circle)
        .insert(TowerRangeCircle(vals.pos.as_uvec2()))
        .insert(TScreen::default());
}

fn tower_laser_cannon() -> ShapeBundle {
    let shape = shapes::Rectangle {
        origin: RectangleOrigin::CustomCenter(Vec2::new(0., TILE_SIZE / 4.)),
        extents: Vec2::new(TILE_SIZE / 6., TILE_SIZE / 2.),
    };
    GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::SILVER),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, TILE_SIZE / 16.),
        },
        Transform {
            translation: Vec3::new(0., 0., 0.1),
            rotation: Quat::from_rotation_z(0.),
            ..Default::default()
        },
    )
}
