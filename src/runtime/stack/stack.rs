use crate::{misc::{id::Id, value::{Value, ValueType}}, runtime::stack::manager::StackManager};

#[derive(Clone)]
pub struct Stack {
    memory: Vec<Value>,
    id: Id,
}

impl Stack {
    pub fn new_with_capacity(id: Id, capacity: usize) -> Self {
        Self {
            memory: Vec::with_capacity(capacity),
            id,
        }
    }

    pub fn push(&mut self, value: Value, manager: &mut StackManager) {
        self.memory.push(value.clone());
        manager.report_metrics(self.memory.len(), ValueType::from_value(&value));
    }

    pub fn pop(&mut self, manager: &mut StackManager) -> Option<Value> {
        let val = self.memory.pop();
        let vtype = val
            .as_ref()
            .map(|v| ValueType::from_value(v))
            .unwrap_or(ValueType::Null);
        manager.report_metrics(self.memory.len(), vtype);
        val
    }

    pub fn swap(&mut self) -> bool {
        let object1 = self.memory.pop();
        let object2 = self.memory.pop();

        if let (Some(object1), Some(object2)) = (object1, object2) {
            self.memory.push(object1);
            self.memory.push(object2);
            return true;
        }
        false
    }

    pub fn copy(&mut self, source: usize, destination: usize) {
        if source == destination {
            return;
        }

        let (left, right) = if source < destination {
            self.memory.split_at_mut(destination)
        } else {
            self.memory.split_at_mut(source)
        };

        let (src, dest) = if source < destination {
            (&left[source], &mut right[0])
        } else {
            (&right[0], &mut left[destination])
        };

        *dest = src.clone();
    }
}