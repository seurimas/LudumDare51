use bevy::ecs::entity::Entities;

use crate::prelude::*;

#[derive(Debug, Component, Clone, Inspectable)]
pub struct Health {
    pub max_health: i32,
    pub health: i32,
    pub dead: bool,
}

impl Health {
    pub fn dies(&mut self) -> bool {
        if self.health <= 0 {
            self.dead = true;
            true
        } else {
            false
        }
    }

    pub fn revive(&mut self) {
        self.health = self.max_health;
        self.dead = false;
    }
}

#[derive(Debug, Clone)]
pub struct DeathEvent(pub Entity, pub Vec2);

pub fn apply_basic_hits(
    mut commands: Commands,
    mut ev_death: EventWriter<DeathEvent>,
    mut ev_bullet_hit: EventReader<BulletHitEvent>,
    mut health_query: Query<(&Transform, &mut Health)>,
    entities: &Entities,
) {
    for BulletHitEvent {
        bullet_entity,
        target_entity,
        bullet_type,
    } in ev_bullet_hit.iter()
    {
        if let Ok((transform, mut target_health)) = health_query.get_mut(*target_entity) {
            target_health.health -= bullet_type.damage();
            if target_health.dies() {
                ev_death.send(DeathEvent(
                    *target_entity,
                    Vec2::new(transform.translation.x, transform.translation.y),
                ));
            }
            if entities.contains(*bullet_entity) {
                commands.entity(*bullet_entity).despawn();
            }
        }
    }
}
