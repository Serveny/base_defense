use std::time::Duration;

use super::{
    tower_base_shape, tower_circle_shape, tower_range_circle_shape, Tower, TowerCannon,
    TowerParent, TowerRangeCircle, TowerValues,
};
use crate::{
    board::visualisation::TILE_SIZE,
    utils::{
        shots::{Shot, TowerStatus},
        visible, BoardPos, Vec2Board,
    },
};
use bevy::color::palettes::css::{DARK_GRAY, RED, SILVER};
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
    is_preview: bool,
) {
    let mut color = RED;
    if is_preview {
        color.alpha = 0.9;
    }
    let transform = Transform::from_translation(vals.pos.to_scaled_vec3(1.));
    cmds.spawn((
        transform,
        Visibility::Inherited,
        TowerParent,
        BoardPos(vals.pos.as_uvec2()),
        TScreen::default(),
    ))
    .with_children(|parent| {
        laser_tower_children::<TScreen>(parent, &vals, color.into(), is_preview)
    })
    .insert(Tower::Laser(vals));
}

fn laser_tower_children<TScreen: Component + Default>(
    parent: &mut ChildBuilder,
    vals: &TowerValues,
    color: Color,
    is_preview: bool,
) {
    // Tower base
    parent.spawn(tower_base_shape(color));

    // Tower circle
    parent.spawn(tower_circle_shape());

    // Tower cannon
    parent.spawn(tower_laser_cannon()).insert(TowerCannon);

    // Range circle
    let range_radius = match is_preview {
        true => vals.range_radius * 2.,
        false => vals.range_radius,
    };
    let range_circle = tower_range_circle_shape(range_radius, color, visible(is_preview));
    parent
        .spawn(range_circle)
        .insert(TowerRangeCircle(vals.pos.as_uvec2()))
        .insert(TScreen::default());
}

fn tower_laser_cannon() -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::CustomCenter(Vec2::new(0., TILE_SIZE / 4.)),
                extents: Vec2::new(TILE_SIZE / 6., TILE_SIZE / 2.),
                radii: None,
            }),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.1),
                rotation: Quat::from_rotation_z(0.),
                ..Default::default()
            },
            ..default()
        },
        Fill::color(SILVER),
        Stroke::new(DARK_GRAY, TILE_SIZE / 16.),
    )
}
