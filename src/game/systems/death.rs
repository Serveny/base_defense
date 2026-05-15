use crate::balance::ENEMY_KILL_MATERIAL_REWARD_DIVISOR;
use crate::game::{
    actions::{explosions::ExplosionMessage, resources::ResourcesMessage},
    enemies::Enemy,
    statistics::EnemyKillCount,
};
use bevy::prelude::*;

pub fn death_system(
    mut cmds: Commands,
    mut expl_actions: MessageWriter<ExplosionMessage>,
    mut res_actions: MessageWriter<ResourcesMessage>,
    mut kill_count: ResMut<EnemyKillCount>,
    q_enemies: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in q_enemies.iter() {
        if enemy.health <= 0. {
            expl_actions.write(ExplosionMessage::death(enemy));
            res_actions.write(ResourcesMessage::Materials(
                enemy.health_max / ENEMY_KILL_MATERIAL_REWARD_DIVISOR,
                enemy.pos,
            ));
            cmds.entity(entity).try_despawn();
            kill_count.0 += 1;
        }
    }
}
