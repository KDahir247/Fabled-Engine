use serde::*;

macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Primitive {
    Nil(()),
    UnsignedInt8(u8),
    SignedInt8(i8),
    UnsignedInt16(u16),
    SignedInt16(i16),
    UnsignedInt32(i32),
    UnsignedInt64(u64),
    SignedInt64(i64),
}
