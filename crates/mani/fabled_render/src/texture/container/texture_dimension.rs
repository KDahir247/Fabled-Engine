#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureDimension {
    D1,
    D2,
    D3,
    Cube,
}

impl Default for TextureDimension {
    fn default() -> Self {
        Self::D2
    }
}
