#[repr(u8)]
#[derive(Debug, Clone)]
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

    // Boolean
    Boolean(bool),

    // Strings & Characters
    String(String),
    Character(u8),

    // Nothing
    Null,
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ValueType {
    // Signed values
    Short,
    Int,
    Long,

    // Unsigned values
    UnsignedShort,
    UnsignedInt,
    UnsignedLong,

    // Floats
    Half,
    Float,
    Double,

    // Boolean
    Boolean,

    // Strings & Characters
    String,
    Character,

    // Nothing
    Null,
}

impl ValueType {
    pub fn from_value(value: &Value) -> ValueType {
        match value {
            Value::Short(_) => ValueType::Short,
            Value::Int(_) => ValueType::Int,
            Value::Long(_) => ValueType::Long,
            Value::UnsignedShort(_) => ValueType::UnsignedShort,
            Value::UnsignedInt(_) => ValueType::UnsignedInt,
            Value::UnsignedLong(_) => ValueType::UnsignedLong,
            Value::Half(_) => ValueType::Half,
            Value::Float(_) => ValueType::Float,
            Value::Double(_) => ValueType::Double,
            Value::Boolean(_) => ValueType::Boolean,
            Value::String(_) => ValueType::String,
            Value::Character(_) => ValueType::Character,
            Value::Null => ValueType::Null,
        }
    }
}
