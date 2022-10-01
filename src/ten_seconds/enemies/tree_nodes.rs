use crate::prelude::*;

use super::ai::*;

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
        audit: &mut Option<BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        if let Some(next_tile) = model.shortest_path.as_ref().and_then(|path| path.0.get(1)) {
            let direction = Vec2::new(
                (next_tile.0 - model.tile.0) as f32,
                (next_tile.1 - model.tile.1) as f32,
            );
            controller.move_towards = Some(direction);
            BehaviorTreeState::Complete
        } else {
            println!("{:?}", model);
            BehaviorTreeState::Failed
        }
    }
}
