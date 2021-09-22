use crate::material::{MaterialAttributes, MaterialBranch};
pub use serde::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialTree {
    branch: Vec<MaterialBranch>,
    // 8 u8 for more metadata
}

impl Index<usize> for MaterialTree {
    type Output = MaterialBranch;

    fn index(&self, index: usize) -> &Self::Output {
        &self.branch[index]
    }
}

impl IndexMut<usize> for MaterialTree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.branch[index]
    }
}

impl Default for MaterialTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialTree {
    pub fn new() -> Self {
        Self {
            branch: vec![
                MaterialBranch::new(MaterialAttributes::Scalar),
                MaterialBranch::new(MaterialAttributes::Vector),
                MaterialBranch::new(MaterialAttributes::Matrix),
                MaterialBranch::new(MaterialAttributes::Image),
                MaterialBranch::new(MaterialAttributes::Sampler),
            ],
        }
    }
}
