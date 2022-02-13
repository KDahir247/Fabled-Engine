use crate::material::MaterialTarget;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaterialNode {
    pub ty: MaterialTarget,
    pub value_group: u32,
    pub value_binding: u32,
}
