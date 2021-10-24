use fabled_render::material::MaterialType;
use fabled_render::texture::Texture;

pub struct MaterialMetadata<'a> {
    pub texture: Texture<'a>,
    pub material: MaterialType,
}
