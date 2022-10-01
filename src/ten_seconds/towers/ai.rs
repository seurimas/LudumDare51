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
}

#[derive(Debug)]
pub struct TowerWorldView {
    pub delta_seconds: f32,
    pub location: Vec2,
    pub enemies: Vec<(Vec2, EnemyType, EnemyImpulses)>,
    pub my_type: TowerType,
    pub time_since_shot: f32,
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
        cooldowns.pass_time(delta_seconds);
        let model = TowerWorldView {
            delta_seconds,
            location: get_location_from_transform(transform),
            enemies: enemies.clone(),
            my_type: *tower_type,
            time_since_shot: cooldowns.time_since_shot,
        };
        let mut new_impulses = TowerImpulses::default();
        behavior_tree.resume_with(&model, &mut new_impulses, &mut None, &mut None);
        *impulses = new_impulses;
    }
}

pub fn shoot_for_towers(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut towers_query: Query<(&Transform, &TowerImpulses, &mut TowerCooldowns)>,
) {
    for (transform, impulses, mut cooldowns) in towers_query.iter_mut() {
        if let Some((bullet_type, velocity, lifetime)) = impulses.fire_now {
            let mut bullet_transform = Transform::default();
            bullet_transform.translation = transform.translation.clone();
            bullet_transform.rotation = Quat::from_rotation_z(velocity.angle_between(Vec2::X));
            spawn_bullet(
                &mut commands,
                &sprites,
                bullet_transform,
                bullet_type,
                velocity,
                lifetime,
            );
            cooldowns.time_since_shot = 0.;
        }
    }
}
