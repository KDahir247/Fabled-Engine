use crate::material::MaterialValue;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialNode {
    pub value_detail: MaterialValue,
    pub value_group: u32,
    pub value_binding: u32,
}
