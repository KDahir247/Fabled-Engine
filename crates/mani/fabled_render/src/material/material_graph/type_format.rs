#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypeFormat {
    Undefined,
    Uint32,
    SInt32,
    Float32,
    Boolean,
}

impl Default for TypeFormat {
    fn default() -> Self {
        TypeFormat::Undefined
    }
}
