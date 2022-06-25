use crate::mesh::container::Mesh;

// todo we can remove this.
#[derive(Debug)]
#[repr(C, align(16))]
pub struct Model {
    pub meshes: Vec<Mesh>,
}
