use crate::misc::value::ValueType;

#[derive(Debug, Clone)]
pub enum MemoryAction {
    Push { value_type: ValueType },
    Pop,
    Swap,
    Copy { source: usize, destination: usize },
}