use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;
pub type Material<'a> = (Texture<'a>, MaterialParameter);


pub struct MaterialMetadata<'a> {
    pub materials: Vec<Material<'a>>,
}

impl<'a> Default for MaterialMetadata<'a> {
    fn default() -> Self {
        Self {
            materials: Vec::default(),
        }
    }
}
