use crate::prelude::*;

pub fn die_enemies(
    mut commands: Commands,
    mut ev_death: EventReader<DeathEvent>,
    is_enemy: Query<(), With<EnemyType>>,
) {
    for DeathEvent(entity) in ev_death.iter() {
        if is_enemy.get(*entity).is_ok() {
            commands.entity(*entity).despawn();
        }
    }
}
