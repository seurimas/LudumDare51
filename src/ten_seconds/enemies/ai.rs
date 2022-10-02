use bevy::utils::HashMap;
use pathfinding::prelude::{astar, astar_bag, astar_bag_collect, AstarSolution};

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

pub struct EnemyWorldView {
    pub field_offset_size: (Vec2, f32),
    pub location: Vec2,
    pub tile: FieldLocation,
    pub my_type: EnemyType,
    pub distance_from_goal: i32,
    pub shortest_paths: Option<(Vec<Vec<FieldLocation>>, i32)>,
    pub neighbor_towers: Vec<(Entity, TowerType)>,
}

#[derive(Component, Deref, DerefMut)]
pub struct EnemyBehaviorTree(
    pub Box<dyn BehaviorTree<Model = EnemyWorldView, Controller = EnemyImpulses> + Send + Sync>,
);

type MemoizedPaths = HashMap<FieldLocation, (f64, (Vec<Vec<FieldLocation>>, i32))>;
#[derive(Default, Deref, DerefMut)]
pub struct BestPaths(pub MemoizedPaths);

#[derive(Default, Deref, DerefMut)]
pub struct BestSeekerPaths(pub MemoizedPaths);

pub fn think_for_enemies(
    time: Res<Time>,
    field: Res<Field>,
    mut best_paths: ResMut<BestPaths>,
    mut best_seeker_paths: ResMut<BestSeekerPaths>,
    mut enemies_query: Query<(
        &Transform,
        &EnemyType,
        &mut EnemyBehaviorTree,
        &mut EnemyImpulses,
    )>,
) {
    let now = time.seconds_since_startup();
    for (enemy_transform, enemy_type, mut behavior_tree, mut impulses) in enemies_query.iter_mut() {
        let location = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.y);
        if let Some(tile) = get_tile_from_location(location, &field) {
            let tile = FieldLocation(tile.0, tile.1);
            let shortest_paths =
                if *enemy_type == EnemyType::Seeker || *enemy_type == EnemyType::Thief {
                    get_shortest_path(
                        tile,
                        &field,
                        |n| field.get_pathable_neighbors_flat_cost(n),
                        now,
                        &mut best_seeker_paths,
                    )
                } else {
                    get_shortest_path(
                        tile,
                        &field,
                        |n| field.get_pathable_neighbors(n),
                        now,
                        &mut best_paths,
                    )
                };
            let neighbor_towers = get_neighbor_towers(&field, tile);
            let view = EnemyWorldView {
                field_offset_size: (field.offset, field.tile_size),
                distance_from_goal: field.estimate_distance_to_goal(&tile),
                my_type: *enemy_type,
                neighbor_towers,
                shortest_paths,
                location,
                tile,
            };
            let mut new_impulses: EnemyImpulses = Default::default();
            behavior_tree.resume_with(&view, &mut new_impulses, &mut None, &mut None);
            *impulses = new_impulses;
        }
    }
}

fn get_shortest_path(
    tile: FieldLocation,
    field: &Res<Field>,
    get_neighbors: impl FnMut(&FieldLocation) -> Vec<(FieldLocation, i32)>,
    now: f64,
    memorized: &mut MemoizedPaths,
) -> Option<(Vec<Vec<FieldLocation>>, i32)> {
    if let Some((last_calculated, paths)) = memorized.get(&tile) {
        if now - last_calculated < 1. {
            return Some(paths.clone());
        }
    }
    let shortest = astar_bag_collect(
        &tile,
        get_neighbors,
        |n| field.estimate_distance_to_goal(n),
        |n| field.is_in_goal(n),
    );
    if let Some(paths) = &shortest {
        memorized.insert(tile, (now, paths.clone()));
    }
    shortest
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
