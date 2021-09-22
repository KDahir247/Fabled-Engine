#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureAspect {
    All,
    StencilOnly,
    DepthOnly,
}
