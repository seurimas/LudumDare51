use pathfinding::prelude::astar;

use crate::{prelude::*, ten_seconds::field::FieldLocationContents};

pub fn think_for_enemies(
    field: Res<Field>,
    enemies_query: Query<(&Transform, &EnemyType)>,
    tower_query: Query<(&TowerType)>,
    location_query: Query<(&FieldLocation, &FieldLocationContents)>,
) {
    for (enemy_transform, enemy_type) in enemies_query.iter() {
        let location = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
        if let Some(tile) = get_tile_from_location(location, &field) {
            let tile = FieldLocation(tile.0, tile.1);
            let shortest_path = astar(
                &tile,
                |n| field.get_neighbors(n),
                |n| field.estimate_distance_to_goal(n),
                |n| field.is_in_goal(n),
            );
            let neighbors = field
                .get_neighbors(&tile)
                .iter()
                .map(|neighbor| field.get_contents(&neighbor.0));
        }
    }
}
