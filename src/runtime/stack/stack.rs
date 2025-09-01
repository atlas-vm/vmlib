use crate::{
    misc::{id::Id, value::Value},
};

#[derive(Clone)]
pub struct Stack {
    pub memory: Vec<Value>,
    pub id: Id,
}

impl Stack {
    pub fn new_with_capacity(id: Id, capacity: usize) -> Self {
        Self {
            memory: Vec::with_capacity(capacity),
            id,
        }
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }

    pub fn is_empty(&self) -> bool {
        self.memory.is_empty()
    }
}