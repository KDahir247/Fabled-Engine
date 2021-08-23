use crate::material::Attributes;
pub use serde::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialTree {
    pub name: String,
    pub shader: String,
    pub attributes: [Attributes; 5], // ulid store it an identifier that will identify the id of the attribute.
                                     //The attribute (Scalar, Vector, Matrix, Pointer, ValuePointer, Array, Struct, Image, Sampler, will have an id if the id and the identifier match then it will be appended)
}

impl MaterialTree {
    pub fn new(name: String, shader: String) -> Self {
        Self {
            name,
            shader,
            attributes: [
                Attributes::Scalar(Vec::new()),
                Attributes::Vector(Vec::new()),
                Attributes::Matrix(Vec::new()),
                Attributes::Image(Vec::new()),
                Attributes::Sampler(Vec::new()),
            ],
        }
    }
}
