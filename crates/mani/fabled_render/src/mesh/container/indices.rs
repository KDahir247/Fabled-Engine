use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub enum Indices {
    U16(SmallVec<[u16; 32]>),
    U32(SmallVec<[u32; 16]>),
}

impl Default for Indices {
    fn default() -> Self {
        Indices::U16(SmallVec::new())
    }
}
