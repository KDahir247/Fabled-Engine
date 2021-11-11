use crate::material::{TextureBlending, TextureOptions};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Texture<'a> {
    pub texture: Option<std::borrow::Cow<'a, str>>,
    pub texture_option: TextureOptions,
    pub texture_blending: TextureBlending,
}