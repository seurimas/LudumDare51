use crate::prelude::*;

use super::ai::*;

#[derive(Debug, Clone)]
pub struct FireBulletNode {
    pub name: String,
    pub bullet_type: BulletType,
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

    fn reset(self: &mut Self, _model: &Self::Model) {}

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        if model.time_since_shot <= self.cooldown {
            BehaviorTreeState::Waiting
        } else {
            for (enemy_location, enemy_type, enemy_impulses) in &model.enemies {
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
