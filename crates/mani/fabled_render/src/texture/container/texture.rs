use crate::material::{TextureBlending, TextureOptions};

pub struct Texture<'a> {
    pub texture: std::borrow::Cow<'a, str>,
    pub texture_option: TextureOptions,
    pub texture_blending: TextureBlending,
}
