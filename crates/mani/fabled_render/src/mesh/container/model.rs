use crate::mesh::container::Mesh;

// todo remove
#[derive(Debug)]
#[repr(C, align(16))]
pub struct Model {
    pub meshes: Vec<Mesh>,
}
