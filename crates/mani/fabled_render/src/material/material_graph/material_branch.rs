use crate::material::{MaterialAttributes, MaterialKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialBranch {
    keys: Vec<MaterialKey>,
    attribute: MaterialAttributes,
}

impl MaterialBranch {
    pub fn new(attribute: MaterialAttributes) -> Self {
        Self {
            attribute,
            keys: Vec::new(),
        }
    }

    pub fn with_capacity(attribute: MaterialAttributes, capacity: usize) -> Self {
        Self {
            attribute,
            keys: Vec::with_capacity(capacity),
        }
    }

    pub fn add_to_keys(&mut self, key: MaterialKey) {
        self.keys.push(key)
    }
}
