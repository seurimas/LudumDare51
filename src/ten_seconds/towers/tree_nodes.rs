use crate::{prelude::*, ten_seconds::enemies::ai::EnemyImpulses};

use super::ai::*;

#[derive(Debug, Clone)]
pub struct RotatingAssistNode {
    pub name: String,
    pub idx: usize,
}

impl BehaviorTree for RotatingAssistNode {
    type Model = TowerWorldView;
    type Controller = TowerImpulses;

    fn get_name(self: &Self) -> &String {
        &self.name
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        self.idx = 0;
    }

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<&mut BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        if model.neighbor_towers.len() > 0 {
            if model.has_ammo {
                controller.assist =
                    Some(model.neighbor_towers[self.idx % model.neighbor_towers.len()].0);
                BehaviorTreeState::Waiting
            } else {
                BehaviorTreeState::Complete
            }
        } else {
            BehaviorTreeState::Failed
        }
    }
}

#[derive(Debug, Clone)]
pub struct FireBulletNode {
    pub name: String,
    pub bullet_type: BulletType,
    pub fired: bool,
    pub speed: f32,
    pub cooldown: f32,
    pub lifetime: f32,
}

impl BehaviorTree for FireBulletNode {
    type Model = TowerWorldView;
    type Controller = TowerImpulses;

    fn get_name(self: &Self) -> &String {
        &self.name
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        self.fired = false;
    }

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<&mut BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(&self.name);
        if self.fired {
            audit.mark(&"FireConfirm".to_string());
            audit.exit(&self.name, BehaviorTreeState::Complete);
            return BehaviorTreeState::Complete;
        }
        if model.time_since_shot <= self.cooldown || !model.has_ammo {
            audit.mark(&"Cooldown".to_string());
            audit.exit(&self.name, BehaviorTreeState::Waiting);
            BehaviorTreeState::Waiting
        } else {
            if let Some((enemy_location, enemy_type, enemy_impulses)) =
                get_closest_enemy(model.location, &model.enemies)
            {
                if enemy_location.distance_squared(model.location)
                    > (self.lifetime * self.lifetime * self.speed * self.speed)
                {
                    audit.mark(&"Too far".to_string());
                    audit.exit(&self.name, BehaviorTreeState::Waiting);
                    return BehaviorTreeState::Waiting;
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
                    self.fired = true;
                }
                audit.mark(&"Fired".to_string());
                audit.exit(&self.name, BehaviorTreeState::Waiting);
                BehaviorTreeState::Waiting
            } else {
                audit.exit(&self.name, BehaviorTreeState::Complete);
                BehaviorTreeState::Complete
            }
        }
    }
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
