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
use bevy::color::palettes::css::{DARK_GRAY, PURPLE, SILVER};
use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};

impl Tower {
    pub fn rocket(pos: Vec2Board) -> Self {
        Self::Rocket(TowerValues::rocket(pos))
    }
}

impl TowerValues {
    pub fn rocket(pos: Vec2Board) -> Self {
        use super::super::shots::rocket;
        Self {
            pos,
            range_radius: rocket::INIT_RANGE_RADIUS,
            shot: Shot::rocket(pos),
            reload_duration: Duration::from_secs_f32(5.),
            shoot_duration: Duration::from_secs_f32(1.),

            target_lock: None,
            tower_status: TowerStatus::Waiting,
        }
    }
}

pub(super) fn spawn_rocket_tower<TScreen: Component + Default>(
    cmds: &mut Commands,
    vals: TowerValues,
    is_preview: bool,
) {
    let mut color = PURPLE;
    if is_preview {
        color.alpha = 0.9;
    }
    let transform = Transform::from_translation(vals.pos.to_scaled_vec3(1.));
    cmds.spawn(SpatialBundle::from_transform(transform))
        .with_children(|parent| {
            rocket_tower_children::<TScreen>(parent, &vals, color.into(), is_preview);
        })
        .insert(TowerParent)
        .insert(BoardPos(vals.pos.as_uvec2()))
        .insert(Tower::Rocket(vals))
        .insert(TScreen::default());
}

fn rocket_tower_children<TScreen: Component + Default>(
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
    parent.spawn(tower_rocket_cannon()).insert(TowerCannon);

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

fn tower_rocket_cannon() -> impl Bundle {
    (
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Rectangle {
                origin: RectangleOrigin::CustomCenter(Vec2::new(0., TILE_SIZE / 4.)),
                extents: Vec2::new(TILE_SIZE / 3., TILE_SIZE / 4.),
            }),
            spatial: SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(0., 0., 0.3),
                    rotation: Quat::from_rotation_z(0.),
                    ..Default::default()
                },
                ..default()
            },
            ..default()
        },
        Fill::color(SILVER),
        Stroke::new(DARK_GRAY, TILE_SIZE / 16.),
    )
}
