use crate::misc::value::ValueType;

#[derive(Debug, Clone)]
pub struct StackMetric {
    stack_memory_allocated_maximum: usize,
    stack_memory_allocated_minimum: usize,
    stack_memory_allocated_average: usize,
    stack_memory_allocated_type: ValueType,
    total_stacks: usize,
    total_allocated: usize,
}

impl StackMetric {
    pub fn new() -> Self {
        Self {
            stack_memory_allocated_maximum: isize::MAX as usize,
            stack_memory_allocated_minimum: 0,
            stack_memory_allocated_average: 0,
            stack_memory_allocated_type: ValueType::Null,
            total_stacks: 0,
            total_allocated: 0,
        }
    }

    pub fn update(&mut self, allocated: usize, vtype: ValueType) {
        self.total_stacks += 1;
        self.total_allocated += allocated;

        if allocated > self.stack_memory_allocated_maximum {
            self.stack_memory_allocated_maximum = allocated;
        }

        if allocated < self.stack_memory_allocated_minimum {
            self.stack_memory_allocated_minimum = allocated;
        }

        self.stack_memory_allocated_average =
            self.total_allocated / self.total_stacks;

        self.stack_memory_allocated_type = vtype;
    }

    pub fn suggested_allocation(&self) -> usize {
        if self.total_stacks == 0 {
            16
        } else {
            self.stack_memory_allocated_average.max(4)
        }
    }
}