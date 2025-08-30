#[repr(u8)]
pub enum Value {
    // Signed values
    Short(i16),
    Int(i32),
    Long(i64),

    // Unsigned values
    UnsignedShort(u16),
    UnsignedInt(i32),
    UnsignedLong(i64),

    // Floats
    Half(f16),
    Float(f32),
    Double(f64),

    // Misc
    Boolean(bool),
    String(String),
    Character(u8)
}