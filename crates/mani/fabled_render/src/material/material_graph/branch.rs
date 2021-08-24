use crate::material::{Attributes, MaterialKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialBranch {
    pub attribute: Attributes,
    pub keys: Vec<MaterialKey>, //maybe hashmap ?
}

impl MaterialBranch {
    pub fn new(attribute: Attributes) -> Self {
        Self {
            attribute,
            keys: Vec::new(),
        }
    }

    pub fn with_capacity(attribute: Attributes, capacity: usize) -> Self {
        Self {
            attribute,
            keys: Vec::with_capacity(capacity),
        }
    }
}
