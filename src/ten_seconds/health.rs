use bevy::ecs::entity::Entities;

use crate::prelude::*;

#[derive(Debug, Component, Clone, Inspectable)]
pub struct Health {
    pub max_health: i32,
    pub health: i32,
}

#[derive(Debug, Clone)]
pub struct DeathEvent(pub Entity);

pub fn apply_basic_hits(
    mut commands: Commands,
    mut ev_death: EventWriter<DeathEvent>,
    mut ev_bullet_hit: EventReader<BulletHitEvent>,
    mut health_query: Query<&mut Health>,
    entities: &Entities,
) {
    for BulletHitEvent {
        bullet_entity,
        target_entity,
    } in ev_bullet_hit.iter()
    {
        if let Ok(mut target_health) = health_query.get_mut(*target_entity) {
            target_health.health -= 1;
            if target_health.health == 0 {
                ev_death.send(DeathEvent(*target_entity));
            }
            if entities.contains(*bullet_entity) {
                commands.entity(*bullet_entity).despawn();
            }
        }
    }
}
