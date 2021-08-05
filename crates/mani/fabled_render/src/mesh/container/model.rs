use crate::mesh::container::Mesh;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Model {
    pub meshes: Vec<Mesh>,
}
