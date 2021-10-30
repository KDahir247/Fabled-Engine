#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialParameter {
    None,
    Color([f32; 3]),
    Scalar(f32),
}

impl Default for MaterialParameter {
    fn default() -> Self {
        Self::None
    }
}
