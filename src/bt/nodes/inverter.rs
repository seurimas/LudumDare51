use crate::bt::*;

pub struct Inverter<M, C> {
    name: String,
    node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>,
}

impl<M, C> Inverter<M, C> {
    pub fn new(node: Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>) -> Self {
        Inverter {
            name: get_bt_id(),
            node,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Inverter<M, C> {
    type Model = M;
    type Controller = C;
    fn resume_with(
        self: &mut Self,
        model: &Self::Model,
        controller: &mut Self::Controller,
        gas: &mut Option<i32>,
        mut audit: &mut Option<&mut BehaviorTreeAudit>,
    ) -> BehaviorTreeState {
        audit.enter(self.get_name());
        match self.node.resume_with(model, controller, gas, audit) {
            BehaviorTreeState::Complete => {
                audit.exit(self.get_name(), BehaviorTreeState::Failed);
                return BehaviorTreeState::Failed;
            }
            BehaviorTreeState::Failed => {
                audit.exit(self.get_name(), BehaviorTreeState::Complete);
                return BehaviorTreeState::Complete;
            }
            result => {
                audit.exit(self.get_name(), result);
                // Waiting, NeedsGas
                return result;
            }
        }
    }

    fn reset(self: &mut Self, model: &Self::Model) {
        self.node.reset(model);
    }

    fn get_name(self: &Self) -> &String {
        &self.name
    }
}
