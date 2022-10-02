use pathfinding::prelude::astar;

use crate::{
    prelude::*,
    ten_seconds::{field::FieldLocationContents, towers::TowerCooldowns},
};

#[derive(Component, Debug, Inspectable, Default, Clone)]
pub struct EnemyImpulses {
    pub move_towards: Option<Vec2>,
    pub attack_tower: Option<Entity>,
    pub explode_now: bool,
}

#[derive(Debug)]
pub struct EnemyWorldView {
    pub field_offset_size: (Vec2, f32),
    pub location: Vec2,
    pub tile: FieldLocation,
    pub my_type: EnemyType,
    pub distance_from_goal: i32,
    pub shortest_path: Option<(Vec<FieldLocation>, i32)>,
    pub neighbor_towers: Vec<(Entity, TowerType)>,
}

#[derive(Component, Deref, DerefMut)]
pub struct EnemyBehaviorTree(
    pub Box<dyn BehaviorTree<Model = EnemyWorldView, Controller = EnemyImpulses> + Send + Sync>,
);

pub fn think_for_enemies(
    field: Res<Field>,
    mut enemies_query: Query<(
        &Transform,
        &EnemyType,
        &mut EnemyBehaviorTree,
        &mut EnemyImpulses,
    )>,
) {
    for (enemy_transform, enemy_type, mut behavior_tree, mut impulses) in enemies_query.iter_mut() {
        let location = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
        if let Some(tile) = get_tile_from_location(location, &field) {
            let tile = FieldLocation(tile.0, tile.1);
            let shortest_path = astar(
                &tile,
                |n| {
                    if *enemy_type == EnemyType::Seeker || *enemy_type == EnemyType::Thief {
                        field.get_pathable_neighbors_flat_cost(n)
                    } else {
                        field.get_pathable_neighbors(n)
                    }
                },
                |n| field.estimate_distance_to_goal(n),
                |n| field.is_in_goal(n),
            );
            let neighbor_towers = get_neighbor_towers(&field, tile);
            let view = EnemyWorldView {
                field_offset_size: (field.offset, field.tile_size),
                distance_from_goal: field.estimate_distance_to_goal(&tile),
                my_type: *enemy_type,
                neighbor_towers,
                shortest_path,
                location,
                tile,
            };
            let mut new_impulses: EnemyImpulses = Default::default();
            behavior_tree.resume_with(&view, &mut new_impulses, &mut None, &mut None);
            *impulses = new_impulses;
        }
    }
}

pub fn move_enemies(
    time: Res<Time>,
    mut enemies_query: Query<(&mut Transform, &EnemyType, &EnemyImpulses)>,
) {
    for (mut transform, enemy_type, impulse) in enemies_query.iter_mut() {
        if let Some(movement) = impulse.move_towards {
            let delta = time.delta_seconds() * enemy_type.get_speed();
            transform.translation += Vec3::new(movement.x * delta, movement.y * delta, 0.);
            transform.rotation = get_rotation_towards(movement);
        }
    }
}

pub fn steal_ammo(
    mut enemies_query: Query<(&EnemyType, &EnemyImpulses, &mut Health)>,
    mut ammo_query: Query<&mut TowerCooldowns>,
) {
    for (enemy_type, impulse, mut health) in enemies_query.iter_mut() {
        if let Some(tower_entity) = impulse.attack_tower {
            if let Ok(mut tower_cooldowns) = ammo_query.get_mut(tower_entity) {
                if *enemy_type == EnemyType::Thief {
                    if health.health < health.max_health {
                        if tower_cooldowns.use_ammo() {
                            health.heal(1);
                        }
                    }
                }
            }
        }
    }
}
