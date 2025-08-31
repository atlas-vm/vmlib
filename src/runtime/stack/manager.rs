use std::collections::HashMap;

use crate::{misc::{id::Id, value::ValueType}, runtime::stack::{metric::StackMetric, stack::{MemoryAction, Stack}}};

pub struct StackManager {
    stacks: HashMap<Id, Stack>,
    metrics: StackMetric,
    actions: HashMap<Id, MemoryAction>
}

impl StackManager {
    pub fn new() -> Self {
        Self {
            stacks: HashMap::new(),
            metrics: StackMetric::new(),
            actions: HashMap::new()
        }
    }

    pub fn commit_action(&mut self, id: &Id, action: MemoryAction) {
        self.actions.insert(id.clone(), action);
    }

    pub fn create_stack(&mut self) -> Id {
        let id = Id::new();
        let suggested = self.metrics.suggested_allocation();

        let stack = Stack::new_with_capacity(id.clone(), suggested);
        self.stacks.insert(id.clone(), stack);

        id
    }

    pub fn get_stack(&self, id: &Id) -> Option<&Stack> {
        self.stacks.get(id)
    }

    pub fn get_stack_mut(&mut self, id: &Id) -> Option<&mut Stack> {
        self.stacks.get_mut(id)
    }

    pub fn report_metrics(&mut self, allocated: usize, vtype: ValueType) {
        self.metrics.update(allocated, vtype);
    }
}