#[derive(Debug)]
pub enum Indices {
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl Default for Indices {
    fn default() -> Self {
        Indices::U16(Vec::new())
    }
}

impl From<Vec<usize>> for Indices {
    fn from(primitive: Vec<usize>) -> Self {
        Self::U32(primitive.iter().map(|&x| x as u32).collect())
    }
}
