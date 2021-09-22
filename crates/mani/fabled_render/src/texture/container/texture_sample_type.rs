#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureSampleType {
    Float { filterable: bool },
    Depth,
    Sint,
    Uint,
}
