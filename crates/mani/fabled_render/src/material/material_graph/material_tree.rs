use crate::material::{MaterialBranch, MaterialPrimitiveType};
pub use serde::*;
use std::borrow::Borrow;
use std::ops::{Deref, Index, IndexMut};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialTree {
    // We can add more meta data the metadata should only occupy 64 bytes
    // we want it the metadata for the shader to only fit one cache line.
    branches: [MaterialBranch; 7],
}

impl Index<usize> for MaterialTree {
    type Output = MaterialBranch;

    fn index(&self, index: usize) -> &Self::Output {
        &self.branches[index]
    }
}

impl IndexMut<usize> for MaterialTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.branches[index]
    }
}

impl Deref for MaterialTree {
    type Target = [MaterialBranch; 7];

    fn deref(&self) -> &Self::Target {
        self.branches.borrow()
    }
}

impl Default for MaterialTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialTree {
    pub const fn new() -> Self {
        Self {
            branches: [
                MaterialBranch::new(MaterialPrimitiveType::Scalar),
                MaterialBranch::new(MaterialPrimitiveType::Vector),
                MaterialBranch::new(MaterialPrimitiveType::Matrix),
                MaterialBranch::new(MaterialPrimitiveType::Image),
                MaterialBranch::new(MaterialPrimitiveType::Sampler),
                MaterialBranch::new(MaterialPrimitiveType::Array),
                MaterialBranch::new(MaterialPrimitiveType::Struct),
            ],
        }
    }
}
