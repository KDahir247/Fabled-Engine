use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;

#[derive(Debug)]
pub struct Material<'a> {
    pub texture: Texture<'a>,
    pub mtl_param: MaterialParameter,
}
