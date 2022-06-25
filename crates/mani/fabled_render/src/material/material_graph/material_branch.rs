use crate::material::{MaterialKey, MaterialPrimitiveType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialBranch {
    primitive_ty: MaterialPrimitiveType,
    keys: smallvec::SmallVec<[MaterialKey; 6]>,
}

impl MaterialBranch {
    pub const fn new(attribute: MaterialPrimitiveType) -> Self {
        Self {
            primitive_ty: attribute,
            keys: smallvec::SmallVec::new_const(),
        }
    }

    pub fn with_capacity(attribute: MaterialPrimitiveType, capacity: usize) -> Self {
        Self {
            primitive_ty: attribute,
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
