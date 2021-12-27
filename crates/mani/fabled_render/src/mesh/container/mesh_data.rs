use crate::mesh::container::Vertex;
use crate::mesh::Indices;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Indices,
    pub material_id: u32,
}
