use crate::material::{TextureBlending, TextureOptions};

#[derive(Debug, Default, PartialEq)]
pub struct Texture<'a> {
    pub texture: Option<std::borrow::Cow<'a, str>>,
    pub texture_option: TextureOptions,
    pub texture_blending: TextureBlending,
}
