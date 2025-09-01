pub mod stack;

use crate::{
    misc::{id::Id, instructions::Instruction, value::Value},
    runtime::stack::manager::StackManager,
};

pub struct Runtime {
    manager: StackManager,
    stack_id: Id, // store the ID of a single stack

    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    instruction_pointer: usize,

    id: Id,
}

impl Runtime {
    pub fn new(instructions: Vec<Instruction>, constants: Vec<Value>) -> Self {
        let mut manager = StackManager::new();
        let stack_id = manager.create_stack();

        Self {
            manager,
            stack_id,
            instructions,
            constants,
            instruction_pointer: 0,
            id: Id::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), &str> {
        while self.instruction_pointer < self.instructions.len() {
            let instruction = &self.instructions[self.instruction_pointer];

            match instruction {
                Instruction::Push { constant_id } => {
                    let value = self
                        .constants
                        .get(*constant_id)
                        .ok_or("Unable to get constant")?
                        .clone();
                    self.manager.push(&self.stack_id, value).map_err(|_| "Failed to push value")?;
                }
                Instruction::Pop {} => {
                    self.manager.pop(&self.stack_id).map_err(|_| "Failed to pop value")?;
                }
                Instruction::Copy { source, destination } => {
                    self.manager.copy(&self.stack_id, *source, *destination)
                        .map_err(|_| "Failed to copy value")?;
                }
                Instruction::Swap {} => {
                    self.manager.swap(&self.stack_id).map_err(|_| "Failed to swap values")?;
                }
                Instruction::Add {} => {
                    let value1 = self.manager
                        .pop(&self.stack_id)
                        .map_err(|_| "Failed to pop first value")?;
                    let value2 = self.manager
                        .pop(&self.stack_id)
                        .map_err(|_| "Failed to pop second value")?;
                    let result = match (value1, value2) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
                        _ => return Err("Unsupported types for addition"),
                    };

                    self.manager
                        .push(&self.stack_id, result)
                        .map_err(|_| "Failed to push result")?;
                }
                Instruction::Subtract {  }=> {
                    let value1 = self.manager
                        .pop(&self.stack_id)
                        .map_err(|_| "Failed to pop first value")?;
                    let value2 = self.manager
                        .pop(&self.stack_id)
                        .map_err(|_| "Failed to pop second value")?;
                    let result = match (value1, value2) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(b - a),
                        (Value::Float(a), Value::Float(b)) => Value::Float(b - a),
                        _ => return Err("Unsupported types for subtraction"),
                    };

                    self.manager
                        .push(&self.stack_id, result)
                        .map_err(|_| "Failed to push result")?;
                }
                Instruction::Jump { target } => {
                    if *target as usize >= self.instructions.len() {
                        return Err("Jump target out of bounds");
                    }
                    self.instruction_pointer = *target as usize;
                    continue;
                }
                _ => {}
            }

            self.instruction_pointer += 1;
        }

        self.manager.delete_stack(&self.stack_id)?;

        Ok(())
    }
}
