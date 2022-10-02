use crate::{
    prelude::*,
    ten_seconds::{bullets::spawn_bullet, enemies::ai::EnemyImpulses},
};

use super::TowerCooldowns;

#[derive(Component, Debug, Inspectable, Default)]
pub struct TowerImpulses {
    pub face_towards: Option<Vec2>,
    pub attack_enemy: Option<Entity>,
    pub fire_now: Option<(BulletType, Vec2, f32)>,
    pub assist: Option<Entity>,
}

#[derive(Debug)]
pub struct TowerWorldView {
    pub delta_seconds: f32,
    pub location: Vec2,
    pub enemies: Vec<(Vec2, EnemyType, EnemyImpulses)>,
    pub my_type: TowerType,
    pub time_since_shot: f32,
    pub has_ammo: bool,
    pub neighbor_towers: Vec<(Entity, TowerType)>,
}

#[derive(Component, Deref, DerefMut)]
pub struct TowerBehaviorTree(
    pub Box<dyn BehaviorTree<Model = TowerWorldView, Controller = TowerImpulses> + Send + Sync>,
);

pub fn think_for_towers(
    field: Res<Field>,
    time: Res<Time>,
    mut towers_query: Query<(
        &Transform,
        &TowerType,
        &mut TowerCooldowns,
        &mut TowerBehaviorTree,
        &mut TowerImpulses,
    )>,
    enemies_query: Query<(&Transform, &EnemyType, &EnemyImpulses)>,
) {
    let delta_seconds = time.delta_seconds();
    let enemies = enemies_query
        .iter()
        .map(|(transform, enemy_type, impulses)| {
            let location = Vec2::new(transform.translation.x, transform.translation.y);
            (location, *enemy_type, impulses.clone())
        })
        .collect::<Vec<(Vec2, EnemyType, EnemyImpulses)>>();
    for (transform, tower_type, mut cooldowns, mut behavior_tree, mut impulses) in
        towers_query.iter_mut()
    {
        let location = get_location_from_transform(transform);
        if let Some(tile) = get_tile_from_location(location, &field) {
            let tile = FieldLocation(tile.0, tile.1);
            cooldowns.pass_time(delta_seconds);
            let model = TowerWorldView {
                delta_seconds,
                location: get_location_from_transform(transform),
                enemies: enemies.clone(),
                my_type: *tower_type,
                time_since_shot: cooldowns.time_since_shot,
                has_ammo: cooldowns.has_ammo(),
                neighbor_towers: get_neighbor_towers(&field, tile),
            };
            let mut new_impulses = TowerImpulses::default();
            behavior_tree.resume_with(&model, &mut new_impulses, &mut None, &mut None);
            *impulses = new_impulses;
        }
    }
}

pub fn shoot_for_towers(
    mut commands: Commands,
    sprites: Res<Sprites>,
    sounds: Res<Sounds>,
    mut towers_query: Query<(&Transform, &TowerImpulses, &mut TowerCooldowns)>,
    audio: Res<Audio>,
) {
    for (transform, impulses, mut cooldowns) in towers_query.iter_mut() {
        if let Some((bullet_type, velocity, lifetime)) = impulses.fire_now {
            if cooldowns.use_ammo() {
                if velocity.length_squared() < 10. {
                    println!("{:?}", velocity);
                }
                let mut bullet_transform = Transform::default();
                bullet_transform.translation = transform.translation.clone();
                bullet_transform.rotation = get_rotation_towards(velocity);
                spawn_bullet(
                    &mut commands,
                    &sprites,
                    bullet_transform,
                    bullet_type,
                    velocity,
                    lifetime,
                );
                cooldowns.time_since_shot = 0.;
                if bullet_type.damage() > 1 {
                    audio.play_with_settings(
                        sounds.shoot_small.clone(),
                        PlaybackSettings::ONCE.with_volume(rand::random::<f32>() * 0.25 + 0.75),
                    );
                } else {
                    audio.play_with_settings(
                        sounds.shoot_small.clone(),
                        PlaybackSettings::ONCE.with_volume(rand::random::<f32>() * 0.5 + 0.5),
                    );
                }
            }
        }
    }
}

pub fn turn_for_towers(mut towers_query: Query<(&mut Transform, &TowerImpulses)>) {
    for (mut transform, impulse) in towers_query.iter_mut() {
        if let Some(turn) = impulse.face_towards {
            transform.rotation = get_rotation_towards(turn);
        }
    }
}

pub fn assist_towers(
    mut towers_query: Query<(Entity, &TowerImpulses)>,
    mut ammo_query: Query<&mut TowerCooldowns>,
) {
    for (tower_entity, impulse) in towers_query.iter_mut() {
        if let Some(assisted) = impulse.assist {
            let mut can_assist = false;
            if let Ok(assisted_ammo) = ammo_query.get(assisted) {
                if !assisted_ammo.can_gain_ammo() {
                    continue;
                }
            }
            if let Ok(mut self_ammo) = ammo_query.get_mut(tower_entity) {
                can_assist = self_ammo.use_ammo();
            }
            if can_assist {
                if let Ok(mut assisted_ammo) = ammo_query.get_mut(assisted) {
                    assisted_ammo.ammo_left += 1;
                }
            }
        }
    }
}
