#[repr(u8)]
pub enum Instruction {
    // Stack / Memory
    Push { constant_id: usize },
    Pop {},
    Copy { source: usize, destination: usize },
    Load { local: usize },
    Store { local: usize },
    Swap {},

    // Arithmetic / Logic (stack-based)
    Add {}, Subtract {}, Multiply {}, Divide {},
    And {}, Or {}, Xor {}, Not {},

    // Control Flow
    Jump { target: u64 },
    JumpIf { target: u64 },
    Call { function_id: u64 },
    Return {},

    // Error Handling
    Try { handler: Option<u64> },
    Throw {},
    EndTry {},

    // Concurrency / Messaging
    Send { pid: u64 },
    Receive {},
    Spawn { function_id: u64 },
    CurrentFid {},
}
