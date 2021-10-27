#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextureBlending {
    /// Set both horizontal and vertical (UV) texture blending to false.
    None,
    /// Set horizontal (U) texture blending to true. Set vertical (V) texture
    /// blending to false
    BlendU,
    /// Set vertical (V) texture blending to true. Set horizontal (U) texture
    /// blending to false
    BlendV,
    // Set both horizontal and vertical (UV) texture blending to true.
    BlendUV,
}

impl Default for TextureBlending {
    fn default() -> Self {
        Self::None
    }
}
