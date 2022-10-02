use crate::{prelude::*, ten_seconds::enemies::ai::EnemyImpulses};

use super::ai::*;

#[derive(Debug, Clone)]
pub struct FireBulletNode {
    pub name: String,
    pub bullet_type: BulletType,
    pub speed: f32,
    pub cooldown: f32,
    pub lifetime: f32,
}

fn get_closest_enemy(
    my_location: Vec2,
    enemies: &Vec<(Vec2, EnemyType, EnemyImpulses)>,
) -> Option<&(Vec2, EnemyType, EnemyImpulses)> {
    let mut best_index = 0;
    let mut best_distance_squared = f32::INFINITY;
    for index in 0..enemies.len() {
        let enemy_location = enemies[index].0;
        let distance_squared = my_location.distance_squared(enemy_location);
        if distance_squared < best_distance_squared {
            best_index = index;
            best_distance_squared = distance_squared;
        }
    }
    enemies.get(best_index)
}

impl BehaviorTree for FireBulletNode {
    type Model = TowerWorldView;
    type Controller = TowerImpulses;

    fn get_name(self: &Self) -> &String {
        &self.name
    }

    fn reset(self: &mut Self, _model: &Self::Model) {}

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        if model.time_since_shot <= self.cooldown || !model.has_ammo {
            BehaviorTreeState::Waiting
        } else {
            if let Some((enemy_location, enemy_type, enemy_impulses)) =
                get_closest_enemy(model.location, &model.enemies)
            {
                if enemy_location.distance_squared(model.location)
                    > (self.lifetime * self.lifetime * self.speed * self.speed)
                {
                    return BehaviorTreeState::Failed;
                }
                let target_velocity = enemy_impulses
                    .move_towards
                    .map(|direction| direction * enemy_type.get_speed())
                    .unwrap_or_default();
                if let Some(shoot_dir) =
                    lead_shot(self.speed, model.location, *enemy_location, target_velocity)
                {
                    controller.face_towards = Some(shoot_dir);
                    controller.fire_now =
                        Some((self.bullet_type, shoot_dir * self.speed, self.lifetime));
                }
            }
            BehaviorTreeState::Complete
        }
    }
}
