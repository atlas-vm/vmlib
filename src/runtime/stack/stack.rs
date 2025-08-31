use crate::{
    misc::{id::Id, value::{Value, ValueType}},
    runtime::stack::manager::StackManager,
};

#[derive(Debug, Clone)]
pub enum MemoryAction {
    Push { value_type: ValueType },
    Pop,
    Swap,
    Copy { source: usize, destination: usize },
}

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

    pub fn push(&mut self, manager: &mut StackManager, value: Value) -> Result<(), ()> {
        let value_type = ValueType::from_value(&value);
        self.memory.push(value);

        manager.commit_action(&self.id, MemoryAction::Push { value_type: value_type.clone() });
        manager.report_metrics(self.memory.len(), value_type);
        
        Ok(())
    }

    pub fn pop(&mut self, manager: &mut StackManager) -> Result<Value, ()> {
        let value = self.memory.pop();
        if let Some(value) = value {
            manager.commit_action(&self.id, MemoryAction::Pop);
            let value_type = ValueType::from_value(&value);
            manager.report_metrics(self.memory.len(), value_type);
            return Ok(value);
        }

        Err(())
    }

    pub fn swap(&mut self, manager: &mut StackManager) -> Result<(), ()> {
        if self.memory.len() < 2 {
            return Err(());
        }

        let len = self.memory.len();
        self.memory.swap(len - 1, len - 2);

        manager.commit_action(&self.id, MemoryAction::Swap);
        
        Ok(())
    }

    pub fn copy(&mut self, manager: &mut StackManager, source: usize, destination: usize) -> Result<(), ()> {
        if source >= self.memory.len() || destination >= self.memory.len() || source == destination {
            return Err(());
        }

        let value = self.memory[source].clone();
        self.memory[destination] = value;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }
}