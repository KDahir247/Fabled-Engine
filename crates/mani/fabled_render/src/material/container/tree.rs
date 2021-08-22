use crate::material::Attributes;
pub use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialTree {
    pub material_name: String,
    pub shader: String,
    pub attributes: [Attributes; 9],
}

impl MaterialTree {
    pub fn new(material_name: String, shader: String) -> Self {
        Self {
            material_name,
            shader,
            attributes: [
                Attributes::Scalar(Vec::new()),
                Attributes::Vector(Vec::new()),
                Attributes::Matrix(Vec::new()),
                Attributes::Pointer(Vec::new()),
                Attributes::ValuePointer(Vec::new()),
                Attributes::Array(Vec::new()),
                Attributes::Struct(Vec::new()),
                Attributes::Image(Vec::new()),
                Attributes::Sampler(Vec::new()),
            ],
        }
    }
}
