#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack / Memory
    Push { constant_id: usize },
    Pop {},
    Copy { source: usize, destination: usize },
    Swap {},

    // Arithmetic / Logic (stack-based)
    Add {},
    Subtract {},
    Multiply {},
    Divide {},
    And {},
    Or {},
    Xor {},
    Not {},

    // Control Flow
    Jump { target: u64 },
    JumpIf { target: u64 },
    Call { fid: u64 },
    Return {},

    // Error Handling
    Try { handler: Option<u64> },
    Throw {},
    EndTry {},

    // Concurrency / Messaging
    Send { fid: u64 },
    Receive {},
    Spawn { fid: u64 },
    CurrentFid {},
}
