use crate::prelude::*;

pub fn die_enemies(
    mut commands: Commands,
    mut field: ResMut<Field>,
    mut ev_death: EventReader<DeathEvent>,
    enemy_query: Query<&EnemyType>,
) {
    for DeathEvent(entity, location) in ev_death.iter() {
        if let Ok(enemy_type) = enemy_query.get(*entity) {
            if let Some(location) = get_tile_from_location(*location, &field) {
                field.increment_tile_cost(
                    &FieldLocation(location.0, location.1),
                    enemy_type.get_death_tile_cost(),
                );
            }
            commands.entity(*entity).despawn();
        }
    }
}
