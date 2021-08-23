use crate::material::{EmptyTarget, MaterialTarget};
use serde::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct MaterialNode {
    pub value_group: Option<u32>,
    pub value_binding: Option<u32>,
    pub value: MaterialTarget,
}

impl From<MaterialNode> for EmptyNode {
    fn from(_: MaterialNode) -> Self {
        Self {
            value: EmptyTarget::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct EmptyNode {
    pub value: EmptyTarget,
}

impl Default for EmptyNode {
    fn default() -> Self {
        Self {
            value: EmptyTarget::None,
        }
    }
}
