use crate::material::{Attributes, MaterialBranch, MaterialTargetFormat};
pub use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialTree {
    pub shader: String,
    pub branch: Vec<MaterialBranch>,
}

impl MaterialTree {
    pub fn new(shader: String) -> Self {
        Self {
            shader,
            branch: vec![
                MaterialBranch::new(Attributes::Scalar),
                MaterialBranch::new(Attributes::Vector),
                MaterialBranch::new(Attributes::Matrix),
                MaterialBranch::new(Attributes::Image),
                MaterialBranch::new(Attributes::Sampler),
            ],
        }
    }

    pub fn get(&self, target: MaterialTargetFormat) -> Option<usize> {
        match target {
            MaterialTargetFormat::Undefined => None,
            MaterialTargetFormat::UnsignedInt
            | MaterialTargetFormat::SignedInt
            | MaterialTargetFormat::Float
            | MaterialTargetFormat::Boolean => Some(0),
            MaterialTargetFormat::Vector2UnsignedInt
            | MaterialTargetFormat::Vector2SignedInt
            | MaterialTargetFormat::Vector2Float
            | MaterialTargetFormat::Vector2Boolean
            | MaterialTargetFormat::Vector4UnsignedInt
            | MaterialTargetFormat::Vector4SignedInt
            | MaterialTargetFormat::Vector4Float
            | MaterialTargetFormat::Vector4Boolean => Some(1),
            MaterialTargetFormat::Matrix2x2Float | MaterialTargetFormat::Matrix4x4Float => Some(2),
            MaterialTargetFormat::Texture => Some(3),
            MaterialTargetFormat::Sampler => Some(4),
        }
    }
}
