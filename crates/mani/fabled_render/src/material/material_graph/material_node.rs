use crate::material::{EmptyTarget, MaterialTarget};
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct MaterialNode {
    pub ty: MaterialTarget,
    pub value_group: u32,
    pub value_binding: u32,
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
