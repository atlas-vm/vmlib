use std::collections::HashMap;

use crate::{misc::{id::Id, value::ValueType}, runtime::stack::{metric::StackMetric, stack::Stack}};

pub struct StackManager {
    stacks: HashMap<Id, Stack>,
    metrics: StackMetric,
}

impl StackManager {
    pub fn new() -> Self {
        Self {
            stacks: HashMap::new(),
            metrics: StackMetric::new(),
        }
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