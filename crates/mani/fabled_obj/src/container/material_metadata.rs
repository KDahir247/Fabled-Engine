use crate::{IlluminationModel, Material};

#[derive(Debug, Clone)]
pub struct MaterialMetadata<'a> {
    pub illum_model: IlluminationModel,
    pub materials: Vec<Material<'a>>,
}

impl<'a> Default for MaterialMetadata<'a> {
    fn default() -> Self {
        Self {
            illum_model: IlluminationModel::None,
            materials: Vec::default(),
        }
    }
}

impl<'a> MaterialMetadata<'a> {
    pub fn new(size: usize) -> Self {
        Self {
            illum_model: IlluminationModel::None,
            materials: Vec::with_capacity(size),
        }
    }
}
