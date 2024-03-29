use crate::game::{
    actions::{explosions::ExplosionEvent, resources::ResourcesEvent},
    enemies::Enemy,
    statistics::EnemyKillCount,
};
use bevy::prelude::*;

pub fn death_system(
    mut cmds: Commands,
    mut expl_actions: EventWriter<ExplosionEvent>,
    mut res_actions: EventWriter<ResourcesEvent>,
    mut kill_count: ResMut<EnemyKillCount>,
    q_enemies: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in q_enemies.iter() {
        if enemy.health <= 0. {
            expl_actions.send(ExplosionEvent::death(enemy));
            res_actions.send(ResourcesEvent::Materials(enemy.health_max / 10., enemy.pos));
            cmds.entity(entity).despawn_recursive();
            kill_count.0 += 1;
        }
    }
}
