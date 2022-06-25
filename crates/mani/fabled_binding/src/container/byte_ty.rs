#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ByteType {
    Byte,
    KiloByte,
    GigaByte,
}


#[derive(Copy, Clone, Debug)]
pub struct MemorySize {
    pub unit: usize,
}

impl MemorySize {
    pub const fn new(bytes: usize, unit_ty: ByteType) -> MemorySize {
        let conversion = match unit_ty {
            ByteType::Byte => 1,
            ByteType::KiloByte => 1024,
            ByteType::GigaByte => 1024 * 1024,
        };

        Self {
            unit: bytes * conversion,
        }
    }
}
