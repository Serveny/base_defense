use bevy::prelude::*;
pub type Collisions = Vec<Collision>;

#[derive(Debug)]
pub struct Collision {
    pub enemy_before: Entity,
    pub enemy_behind: Entity,
    pub is_critical: bool,
}

impl Collision {
    pub fn new(enemy_before: Entity, enemy_behind: Entity, is_critical: bool) -> Self {
        Self {
            enemy_before,
            enemy_behind,
            is_critical,
        }
    }
}

impl PartialEq for Collision {
    fn eq(&self, other: &Self) -> bool {
        self.enemy_before == other.enemy_before && self.enemy_behind == other.enemy_behind
            || self.enemy_before == other.enemy_behind && self.enemy_behind == other.enemy_before
    }
}
