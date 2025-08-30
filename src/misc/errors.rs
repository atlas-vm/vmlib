#[repr(u8)]
pub enum RuntimeError 
{
    StackOverflow(String),
    InvalidOperation(String),
}

#[repr(u8)]
pub enum LoadError
{
    InvalidVersion(String),
    InvalidChecksum(String),
    InvalidPublisher(String),
}