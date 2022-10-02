use crate::bt::*;

pub struct Selector<M, C> {
    name: String,
    nodes: Vec<Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>>,
    index: Option<usize>,
}

impl<M, C> Selector<M, C> {
    pub fn new(nodes: Vec<Box<dyn BehaviorTree<Model = M, Controller = C> + Send + Sync>>) -> Self {
        Selector {
            name: get_bt_id(),
            nodes,
            index: None,
        }
    }
}

impl<M: 'static, C: 'static> BehaviorTree for Selector<M, C> {
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
        let mut running_index = self.index.unwrap_or(0);
        loop {
            if let Some(node) = self.nodes.get_mut(running_index) {
                let result = node.resume_with(model, controller, gas, audit);
                match result {
                    BehaviorTreeState::Failed => {
                        // Move on to the next node.
                        running_index += 1;
                    }
                    BehaviorTreeState::Complete => {
                        self.index = None;
                        audit.exit(self.get_name(), result);
                        return result;
                    }
                    _ => {
                        // Waiting, NeedsGas
                        self.index = Some(running_index);
                        audit.exit(self.get_name(), result);
                        return result;
                    }
                }
            } else {
                self.index = None;
                audit.exit(self.get_name(), BehaviorTreeState::Failed);
                return BehaviorTreeState::Failed;
            }
        }
    }

    fn reset(self: &mut Self, _parameter: &Self::Model) {
        self.index = None;
    }

    fn get_name(self: &Self) -> &String {
        &self.name
    }
}
