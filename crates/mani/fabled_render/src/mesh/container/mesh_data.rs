use crate::mesh::container::Vertex;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub material_id: u32,
}
