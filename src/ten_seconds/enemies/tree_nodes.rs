use crate::prelude::*;

use super::ai::*;

pub enum EnemyNode {
    Pathfind(PathfindNode),
    Attack(AttackNode),
}

impl UserNodeDefinition for EnemyNode {
    type Model = EnemyWorldView;
    type Controller = EnemyImpulses;

    fn create_node(
        &self,
    ) -> Box<dyn BehaviorTree<Model = Self::Model, Controller = Self::Controller> + Send + Sync>
    {
        match self {
            Self::Pathfind(node) => node.create_node(),
            Self::Attack(node) => node.create_node(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PathfindNode {
    pub name: String,
}

impl BehaviorTree for PathfindNode {
    type Model = EnemyWorldView;
    type Controller = EnemyImpulses;

    fn get_name(self: &Self) -> &String {
        &self.name
    }

    fn reset(self: &mut Self, _model: &Self::Model) {
        // Nothing to do
    }

    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        audit: &mut Option<&mut BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        if let Some(next_tile) = model.shortest_path.as_ref().and_then(|path| path.0.get(1)) {
            let target_location = Vec2::new(
                model.field_offset_size.0.x
                    + model.field_offset_size.1 * (next_tile.0 as f32 + 0.5),
                model.field_offset_size.0.y
                    + model.field_offset_size.1 * (next_tile.1 as f32 + 0.5),
            );
            let direction = (target_location - model.location).normalize();
            controller.move_towards = Some(direction);
            BehaviorTreeState::Complete
        } else {
            BehaviorTreeState::Failed
        }
    }
}

#[derive(Debug, Clone)]
pub struct AttackNode {
    pub name: String,
    pub idx: usize,
}

impl BehaviorTree for AttackNode {
    type Model = EnemyWorldView;
    type Controller = EnemyImpulses;

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
            controller.attack_tower =
                Some(model.neighbor_towers[self.idx % model.neighbor_towers.len()].0);
        }
        BehaviorTreeState::Complete
    }
}
