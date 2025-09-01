use std::{cell::RefCell, collections::HashMap};

use crate::{misc::{id::Id, value::{Value, ValueType}}, runtime::stack::{self, memoryaction::MemoryAction, metric::StackMetric, stack::Stack}};

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

    pub fn get_stack_mut(&mut self, id: &Id) -> Result<&mut Stack, &str> {
        self.stacks.get_mut(id).ok_or("Stack not found")
    }

    pub fn commit_action(&mut self, id: &Id, action: MemoryAction) {
        self.actions.insert(id.clone(), action);

        //println!("Committed action for stack {}: {:?}", id, self.actions.get(id).unwrap());
    }

    pub fn create_stack(&mut self) -> Id {
        let id = Id::new();
        let suggested = self.metrics.suggested_allocation();

        let stack = Stack::new_with_capacity(id.clone(), suggested);
        self.stacks.insert(id.clone(), stack);

        id
    }

    pub fn push(&mut self, id: &Id, value: Value) -> Result<(), &str> {
        {
            let stack = self.stacks.get_mut(id).ok_or("Stack not found")?;
            let memory = &mut stack.memory;

            if memory.try_reserve(1).is_err() {
                memory.clear();
                return Err("Stack overflow");
            }

            memory.push(value.clone());
        }

        let value_type = ValueType::from_value(&value);
        self.commit_action(id, MemoryAction::Push { value_type: value_type.clone() });
        self.report_metrics(self.stacks.get(id).unwrap().memory.len(), value_type);

        Ok(())
    }

    pub fn pop(&mut self, id: &Id) -> Result<Value, &str> {
        let stack_size = {
            let stack = self.stacks.get(id).ok_or("Stack not found")?;
            stack.memory.len()
        };

        let value = {
            let stack = self.stacks.get_mut(id).ok_or("Stack not found")?;
            stack.memory.pop().ok_or("Stack underflow")?
        };

        let value_type = ValueType::from_value(&value);
        self.report_metrics(stack_size, value_type);
        self.commit_action(id, MemoryAction::Pop);

        Ok(value)
    }

    pub fn swap(&mut self, id: &Id) -> Result<(), &str> {
        let stack = self.stacks.get_mut(id).ok_or("Stack not found")?;
        let memory = &mut stack.memory;

        if memory.len() < 2 {
            return Err("Not enough values to swap");
        }

        let len = memory.len();
        memory.swap(len - 1, len - 2);

        self.commit_action(id, MemoryAction::Swap);

        Ok(())
    }

    pub fn copy(&mut self, id: &Id, source: usize, destination: usize) -> Result<(), &str> {
        let stack = self.stacks.get_mut(id).ok_or("Stack not found")?;
        let memory = &mut stack.memory;

        if source >= memory.len() || destination >= memory.len() || source == destination {
            return Err("Invalid source or destination index");
        }

        let value = memory[source].clone();
        memory[destination] = value;

        self.commit_action(id, MemoryAction::Copy { source, destination });

        Ok(())
    }

    pub fn delete_stack(&mut self, id: &Id) -> Result<(), &str> {
        let Some(stack) = self.stacks.get(id) else {
          return Err("Stack not found");
        };

        if !stack.is_empty() {
            return Err("Stack is not empty");
        }

        self.stacks.remove(id);

        Ok(())
    }

    pub fn report_metrics(&mut self, allocated: usize, vtype: ValueType) {
        self.metrics.update(allocated, vtype);
    }
}