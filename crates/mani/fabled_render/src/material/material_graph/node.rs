use crate::material::{EmptyTarget, MaterialTarget};
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct MaterialNode {
    pub value_group: Option<u32>,
    pub value_binding: Option<u32>,
    pub ty: MaterialTarget,
}

impl MaterialNode {
    pub fn get_material_bytes(&self) -> Vec<u8> {
        let byte_buffer = match self.ty {
            MaterialTarget::None => panic!(), //todo
            MaterialTarget::Uint(v) => v.get_bytes(),
            MaterialTarget::Sint(v) => v.get_bytes(),
            MaterialTarget::Float(v) => v.get_bytes(),
            MaterialTarget::Vec2UInt(v) => v.get_bytes(),
            MaterialTarget::Vec2SInt(v) => v.get_bytes(),
            MaterialTarget::Vec2Float(v) => v.get_bytes(),
            MaterialTarget::Vec4UInt(v) => v.get_bytes(),
            MaterialTarget::Vec4SInt(v) => v.get_bytes(),
            MaterialTarget::Vec4Float(v) => v.get_bytes(),
            MaterialTarget::Matrix2x2(v) => v.get_bytes(),
            MaterialTarget::Matrix4x4(v) => v.get_bytes(),
            MaterialTarget::Sampler(v) => v.get_bytes(),
        };

        byte_buffer
    }
}

impl From<MaterialNode> for EmptyNode {
    fn from(_: MaterialNode) -> Self {
        Self {
            ty: EmptyTarget::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct EmptyNode {
    pub ty: EmptyTarget,
}

impl Default for EmptyNode {
    fn default() -> Self {
        Self {
            ty: EmptyTarget::None,
        }
    }
}
