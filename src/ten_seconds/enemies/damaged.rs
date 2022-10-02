use crate::prelude::*;

pub fn die_enemies(
    mut commands: Commands,
    mut wave_status: ResMut<WaveStatus>,
    mut field: ResMut<Field>,
    mut ev_death: EventReader<DeathEvent>,
    mut enemy_query: Query<(&EnemyType, &mut Health)>,
) {
    for DeathEvent(entity, location) in ev_death.iter() {
        if let Ok((enemy_type, mut health)) = enemy_query.get_mut(*entity) {
            if let Some(location) = get_tile_from_location(*location, &field) {
                if *enemy_type == EnemyType::Buster
                    && !can_path_from_spawn_if(&field, |loc| {
                        loc.0 == location.0 && loc.1 == location.1
                    })
                {
                    health.revive();
                    continue;
                } else {
                    field.increment_tile_cost(
                        &FieldLocation(location.0, location.1),
                        enemy_type.get_death_tile_cost(),
                    );
                    wave_status.loot(enemy_type);
                }
            }
            commands.entity(*entity).despawn();
        }
    }
}
