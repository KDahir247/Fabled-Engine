use crate::material::{EmptyTarget, MaterialTarget};
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialNode {
    pub value_name: String,
    pub value_type: naga::TypeInner,
    pub value_group: Option<u32>,
    pub value_binding: Option<u32>,
    pub value: MaterialTarget,
}

impl From<MaterialNode> for EmptyNode {
    fn from(mat_node: MaterialNode) -> Self {
        Self {
            value_name: mat_node.value_name,
            value_type: mat_node.value_type,
            value: EmptyTarget::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyNode {
    pub value_name: String,
    pub value_type: naga::TypeInner,
    pub value: EmptyTarget,
}
