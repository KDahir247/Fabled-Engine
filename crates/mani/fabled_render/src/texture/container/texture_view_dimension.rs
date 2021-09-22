#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}
