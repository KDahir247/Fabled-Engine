use crate::material::{MaterialAttributes, MaterialKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialBranch {
    attribute: MaterialAttributes,
    keys: smallvec::SmallVec<[MaterialKey; 6]>,
}

impl MaterialBranch {
    pub const fn new(attribute: MaterialAttributes) -> Self {
        Self {
            attribute,
            keys: smallvec::SmallVec::new_const(),
        }
    }

    pub fn with_capacity(attribute: MaterialAttributes, capacity: usize) -> Self {
        Self {
            attribute,
            keys: smallvec::SmallVec::with_capacity(capacity),
        }
    }

    pub fn add_to_keys(&mut self, key: MaterialKey) {
        self.keys.push(key)
    }

    pub fn copy_to_keys(&mut self, keys: &[MaterialKey]) {
        self.keys.insert_from_slice(0, keys);
    }
}
